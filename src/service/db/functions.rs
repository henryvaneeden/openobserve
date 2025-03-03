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

use std::sync::Arc;

use crate::common::{
    infra::{
        config::{QUERY_FUNCTIONS, STREAM_FUNCTIONS},
        db as infra_db,
    },
    meta::functions::{StreamFunctionsList, Transform},
    utils::json,
};

pub async fn set(org_id: &str, name: &str, js_func: &Transform) -> Result<(), anyhow::Error> {
    let db = &infra_db::DEFAULT;
    let key = format!("/function/{org_id}/{name}");
    match db
        .put(
            &key,
            json::to_vec(js_func).unwrap().into(),
            infra_db::NEED_WATCH,
        )
        .await
    {
        Ok(_) => {}
        Err(e) => {
            log::error!("Error saving function: {}", e);
            return Err(anyhow::anyhow!("Error saving function: {}", e));
        }
    }

    Ok(())
}

pub async fn get(org_id: &str, name: &str) -> Result<Transform, anyhow::Error> {
    let db = &infra_db::DEFAULT;
    let val = db.get(&format!("/function/{org_id}/{name}")).await?;
    Ok(json::from_slice(&val).unwrap())
}

pub async fn delete(org_id: &str, name: &str) -> Result<(), anyhow::Error> {
    let db = &infra_db::DEFAULT;
    let key = format!("/function/{org_id}/{name}");
    match db.delete(&key, false, infra_db::NEED_WATCH).await {
        Ok(_) => {}
        Err(e) => {
            log::error!("Error deleting function: {}", e);
            return Err(anyhow::anyhow!("Error deleting function: {}", e));
        }
    }
    Ok(())
}

pub async fn list(org_id: &str) -> Result<Vec<Transform>, anyhow::Error> {
    let db = &infra_db::DEFAULT;

    Ok(db
        .list(&format!("/function/{org_id}/"))
        .await?
        .values()
        .map(|val| json::from_slice(val).unwrap())
        .collect())
}

pub async fn watch() -> Result<(), anyhow::Error> {
    let key = "/function/";
    let db = &infra_db::CLUSTER_COORDINATOR;
    let mut events = db.watch(key).await?;
    let events = Arc::get_mut(&mut events).unwrap();
    log::info!("Start watching function");
    loop {
        let ev = match events.recv().await {
            Some(ev) => ev,
            None => {
                log::error!("watch_functions: event channel closed");
                break;
            }
        };
        match ev {
            infra_db::Event::Put(ev) => {
                let item_key = ev.key.strip_prefix(key).unwrap();
                let org_id = &item_key[0..item_key.find('/').unwrap()];
                let item_value: Transform = json::from_slice(&ev.value.unwrap()).unwrap();
                if item_value.streams.is_some() {
                    for stream_fn in item_value.to_stream_transform() {
                        let mut group = STREAM_FUNCTIONS
                            .entry(format!(
                                "{}/{}/{}",
                                org_id, stream_fn.stream_type, stream_fn.stream
                            ))
                            .or_insert(StreamFunctionsList { list: vec![] });
                        if group.list.contains(&stream_fn) {
                            let stream_name =
                                group.list.iter().position(|x| x.eq(&stream_fn)).unwrap();
                            let _ = std::mem::replace(&mut group.list[stream_name], stream_fn);
                        } else {
                            group.list.push(stream_fn);
                        }
                    }
                } else {
                    QUERY_FUNCTIONS.insert(item_key.to_owned(), item_value);
                }
            }
            infra_db::Event::Delete(ev) => {
                let item_key = ev.key.strip_prefix(key).unwrap();
                QUERY_FUNCTIONS.remove(item_key);
            }
            infra_db::Event::Empty => {}
        }
    }
    Ok(())
}

pub async fn cache() -> Result<(), anyhow::Error> {
    let db = &infra_db::DEFAULT;
    let key = "/function/";
    let ret = db.list(key).await?;
    for (item_key, item_value) in ret {
        let item_key = item_key.strip_prefix(key).unwrap();
        let json_val: Transform = json::from_slice(&item_value).unwrap();
        let org_id = &item_key[0..item_key.find('/').unwrap()];
        if json_val.streams.is_some() {
            for stream_fn in json_val.to_stream_transform() {
                let mut group = STREAM_FUNCTIONS
                    .entry(format!(
                        "{}/{}/{}",
                        org_id, stream_fn.stream_type, stream_fn.stream
                    ))
                    .or_insert(StreamFunctionsList { list: vec![] });
                group.list.push(stream_fn);
            }
            let mut func = json_val.clone();
            func.streams = None;
            QUERY_FUNCTIONS.insert(item_key.to_string(), func);
        } else {
            QUERY_FUNCTIONS.insert(item_key.to_string(), json_val);
        }
    }
    log::info!("Functions Cached");
    Ok(())
}

pub async fn reset() -> Result<(), anyhow::Error> {
    let db = &infra_db::DEFAULT;
    let key = "/function/";
    db.delete(key, true, infra_db::NO_NEED_WATCH).await?;
    let key = "/transform/";
    db.delete(key, true, infra_db::NO_NEED_WATCH).await?;
    Ok(())
}
