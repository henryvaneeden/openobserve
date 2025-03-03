// Copyright 2023 Zinc Labs Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use actix_web::{http, web};
use ahash::AHashMap;
use chrono::{Duration, Utc};
use datafusion::arrow::datatypes::Schema;

use super::StreamMeta;
use crate::common::{
    infra::{
        config::{CONFIG, DISTINCT_FIELDS},
        metrics,
    },
    meta::{
        alert::{Alert, Trigger},
        ingestion::{IngestionResponse, StreamStatus},
        stream::StreamParams,
        usage::UsageType,
        StreamType,
    },
    utils::{flatten, json, time::parse_timestamp_micro_from_value},
};
use crate::service::{
    distinct_values, get_formatted_stream_name, ingestion::is_ingestion_allowed,
    ingestion::write_file, usage::report_request_usage_stats,
};

pub async fn ingest(
    org_id: &str,
    in_stream_name: &str,
    body: web::Bytes,
    thread_id: usize,
) -> Result<IngestionResponse, anyhow::Error> {
    let start = std::time::Instant::now();
    let mut stream_schema_map: AHashMap<String, Schema> = AHashMap::new();
    let mut distinct_values = Vec::with_capacity(16);
    let mut stream_params = StreamParams::new(org_id, in_stream_name, StreamType::Logs);
    let stream_name = &get_formatted_stream_name(&mut stream_params, &mut stream_schema_map).await;

    if let Some(value) = is_ingestion_allowed(org_id, Some(stream_name)) {
        return Err(value);
    }

    let mut min_ts =
        (Utc::now() + Duration::hours(CONFIG.limit.ingest_allowed_upto)).timestamp_micros();

    let mut runtime = crate::service::ingestion::init_functions_runtime();

    let mut stream_alerts_map: AHashMap<String, Vec<Alert>> = AHashMap::new();
    let mut stream_status = StreamStatus::new(stream_name);
    let mut trigger: Option<Trigger> = None;

    // Start Register Transforms for stream
    let (local_trans, stream_vrl_map) = crate::service::ingestion::register_stream_transforms(
        org_id,
        StreamType::Logs,
        stream_name,
    );
    // End Register Transforms for stream

    let partition_det =
        crate::service::ingestion::get_stream_partition_keys(stream_name, &stream_schema_map).await;
    let partition_keys = partition_det.partition_keys;
    let partition_time_level = partition_det.partition_time_level;

    // Start get stream alerts
    let key = format!("{}/{}/{}", &org_id, StreamType::Logs, &stream_name);
    crate::service::ingestion::get_stream_alerts(key, &mut stream_alerts_map).await;
    // End get stream alert

    let mut buf: AHashMap<String, Vec<String>> = AHashMap::new();
    let reader: Vec<json::Value> = json::from_slice(&body).unwrap_or({
        let val: json::Value = json::from_slice(&body)?;
        vec![val]
    });
    for item in reader.iter() {
        //JSON Flattening
        let mut value = flatten::flatten(item)?;

        if !local_trans.is_empty() {
            value = crate::service::ingestion::apply_stream_transform(
                &local_trans,
                &value,
                &stream_vrl_map,
                stream_name,
                &mut runtime,
            )?;
        }

        if value.is_null() || !value.is_object() {
            stream_status.status.failed += 1; // transform failed or dropped
            continue;
        }
        // End row based transform

        // get json object
        let local_val = value.as_object_mut().unwrap();

        // handle timestamp
        let timestamp = match local_val.get(&CONFIG.common.column_timestamp) {
            Some(v) => match parse_timestamp_micro_from_value(v) {
                Ok(t) => t,
                Err(e) => {
                    stream_status.status.failed += 1;
                    stream_status.status.error = e.to_string();
                    continue;
                }
            },
            None => Utc::now().timestamp_micros(),
        };
        // check ingestion time
        let earliest_time = Utc::now() + Duration::hours(0 - CONFIG.limit.ingest_allowed_upto);
        if timestamp < earliest_time.timestamp_micros() {
            stream_status.status.failed += 1; // to old data, just discard
            stream_status.status.error = super::get_upto_discard_error();
            continue;
        }
        if timestamp < min_ts {
            min_ts = timestamp;
        }
        local_val.insert(
            CONFIG.common.column_timestamp.clone(),
            json::Value::Number(timestamp.into()),
        );

        let local_trigger = super::add_valid_record(
            &StreamMeta {
                org_id: org_id.to_string(),
                stream_name: stream_name.to_string(),
                partition_keys: &partition_keys,
                partition_time_level: &partition_time_level,
                stream_alerts_map: &stream_alerts_map,
            },
            &mut stream_schema_map,
            &mut stream_status.status,
            &mut buf,
            local_val,
        )
        .await;

        // get distinct_value item
        for field in DISTINCT_FIELDS.iter() {
            if let Some(val) = local_val.get(field) {
                if !val.is_null() {
                    distinct_values.push(distinct_values::DvItem {
                        stream_type: StreamType::Logs,
                        stream_name: stream_name.to_string(),
                        field_name: field.to_string(),
                        field_value: val.as_str().unwrap().to_string(),
                        filter_name: "".to_string(),
                        filter_value: "".to_string(),
                    });
                }
            }
        }

        if local_trigger.is_some() {
            trigger = Some(local_trigger.unwrap());
        }
    }

    // write to file
    let mut stream_file_name = "".to_string();
    let mut req_stats = match tokio::task::spawn_blocking(move || async move {
        write_file(&buf, thread_id, &stream_params, &mut stream_file_name, None).await
    })
    .await
    {
        Ok(result) => result.await,
        Err(err) => {
            log::error!("error writing data : {}", err);
            return Ok(IngestionResponse::new(
                http::StatusCode::OK.into(),
                vec![stream_status],
            ));
        }
    };
    /* if stream_file_name.is_empty() {
        return Ok(IngestionResponse::new(
            http::StatusCode::OK.into(),
            vec![stream_status],
        ));
    } */

    // only one trigger per request, as it updates etcd
    super::evaluate_trigger(trigger, &stream_alerts_map).await;

    // send distinct_values
    if !distinct_values.is_empty() {
        if let Err(e) = distinct_values::write(org_id, distinct_values).await {
            log::error!("Error while writing distinct values: {}", e);
        }
    }

    let time = start.elapsed().as_secs_f64();
    metrics::HTTP_RESPONSE_TIME
        .with_label_values(&[
            "/api/org/ingest/logs/_json",
            "200",
            org_id,
            stream_name,
            StreamType::Logs.to_string().as_str(),
        ])
        .observe(time);
    metrics::HTTP_INCOMING_REQUESTS
        .with_label_values(&[
            "/api/org/ingest/logs/_json",
            "200",
            org_id,
            stream_name,
            StreamType::Logs.to_string().as_str(),
        ])
        .inc();

    req_stats.response_time = start.elapsed().as_secs_f64();
    //metric + data usage
    report_request_usage_stats(
        req_stats,
        org_id,
        stream_name,
        StreamType::Logs,
        UsageType::Json,
        local_trans.len() as u16,
    )
    .await;

    Ok(IngestionResponse::new(
        http::StatusCode::OK.into(),
        vec![stream_status],
    ))
}
