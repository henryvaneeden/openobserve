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

use ahash::AHashMap;
use arrow_schema::Schema;
use once_cell::sync::Lazy;
use regex::{self, Regex};

use crate::common::meta::stream::StreamParams;

pub mod alert_manager;
pub mod alerts;
pub mod compact;
pub mod dashboards;
pub mod db;
pub mod distinct_values;
pub mod enrichment;
pub mod enrichment_table;
pub mod file_list;
pub mod functions;
pub mod ingestion;
pub mod kv;
pub mod logs;
pub mod metrics;
pub mod organization;
pub mod promql;
pub mod router;
pub mod schema;
pub mod search;
pub mod stream;
pub mod syslogs_route;
pub mod traces;
pub mod triggers;
pub mod usage;
pub mod users;

const MAX_KEY_LENGTH: usize = 100;

static RE_CORRECT_STREAM_NAME: Lazy<Regex> = Lazy::new(|| Regex::new(r"[^a-zA-Z0-9_:]+").unwrap());

// format partition key
pub fn format_partition_key(input: &str) -> String {
    let mut output = String::with_capacity(std::cmp::min(input.len(), MAX_KEY_LENGTH));
    for c in input.chars() {
        if output.len() > MAX_KEY_LENGTH {
            break;
        }
        if c.is_alphanumeric() || c == '=' || c == '-' || c == '_' {
            output.push(c);
        }
    }
    output
}

// format stream name
pub async fn get_formatted_stream_name(
    params: &mut StreamParams,
    schema_map: &mut AHashMap<String, Schema>,
) -> String {
    let mut stream_name = params.stream_name.to_string();

    let schema = db::schema::get(&params.org_id, &stream_name, params.stream_type)
        .await
        .unwrap();

    if schema.fields().is_empty() {
        stream_name = RE_CORRECT_STREAM_NAME
            .replace_all(&stream_name, "_")
            .to_string();
        let corrected_schema = db::schema::get(&params.org_id, &stream_name, params.stream_type)
            .await
            .unwrap();
        schema_map.insert(stream_name.to_owned(), corrected_schema);
    } else {
        schema_map.insert(stream_name.to_owned(), schema);
    }
    params.stream_name = stream_name.to_owned().into();

    stream_name
}

// format stream name
pub fn format_stream_name(stream_name: &str) -> String {
    RE_CORRECT_STREAM_NAME
        .replace_all(stream_name, "_")
        .to_string()
}
