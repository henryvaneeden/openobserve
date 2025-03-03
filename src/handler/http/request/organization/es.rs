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

use actix_web::{get, http, put, route, web, HttpRequest, HttpResponse, Result};
use std::io::Error;

#[route("/{org_id}/", method = "GET", method = "HEAD")]
async fn org_index(_org_id: web::Path<String>, req: HttpRequest) -> Result<HttpResponse, Error> {
    // eg.1: User-Agent:[elastic-transport-ruby/8.0.1 (RUBY_VERSION: 3.1.2; linux x86_64; Faraday v1.10.0)]
    // eg.2: Elastic-filebeat/7.17.1 (linux; arm64; 1d05ba86138cfc9a5ae5c0acc64a57b8d81678ff; 2022-02-24 01:00:19 +0000 UTC)
    let mut version = "7.17.1";
    let user_agent = match req.headers().get("User-Agent") {
        Some(user_agent) => user_agent.to_str().unwrap(),
        None => "",
    };
    if user_agent.to_lowercase().contains("elastic") {
        let re = regex::Regex::new(r"(\d+\.\d+\.\d+)").unwrap();
        version = match re.captures(user_agent) {
            Some(caps) => caps.get(1).unwrap().as_str(),
            None => "8.1.0",
        };
    }
    let es_info = r#"{"name":"opensearch","cluster_name":"opensearch-cluster","cluster_uuid":"h3nGzoJ1R12fZz","version":{"number":"0.0.0","build_flavor":"default","build_hash":"0","build_date":"0","build_snapshot":false,"lucene_version":"8.9.0","minimum_wire_version":"7.10.0","minimum_index_compatibility":"8.1.0"},"tagline":"You Know, for Search"}"#;
    let es_info = es_info.replace("0.0.0", version);
    Ok(HttpResponse::Ok()
        .content_type(http::header::ContentType::json())
        .insert_header(("X-Elastic-Product", "Elasticsearch"))
        .body(es_info))
}

#[get("/{org_id}/_license")]
async fn org_license(_org_id: web::Path<String>) -> Result<HttpResponse, Error> {
    let es_info = r#"{"status":"active"}"#;
    Ok(HttpResponse::Ok()
        .content_type(http::header::ContentType::json())
        .insert_header(("X-Elastic-Product", "Elasticsearch"))
        .body(es_info))
}

#[get("/{org_id}/_xpack")]
async fn org_xpack(_org_id: web::Path<String>) -> Result<HttpResponse, Error> {
    let es_info = r#"{"build":{},"features":{},"license":{"status":"active"}}"#;
    Ok(HttpResponse::Ok()
        .content_type(http::header::ContentType::json())
        .insert_header(("X-Elastic-Product", "Elasticsearch"))
        .body(es_info))
}

#[route("/{org_id}/_index_template/{name}", method = "GET", method = "HEAD")]
async fn org_index_template(path: web::Path<(String, String)>) -> Result<HttpResponse, Error> {
    let (_org_id, name) = path.into_inner();
    let es_info = r#"{"index_patterns":["log-*"],"name":"logs","priority":1,"template":{"mappings":{"properties":{"_timestamp":{"aggregatable":false,"highlightable":false,"index":true,"sortable":false,"store":false,"type":"date"}}},"settings":{"number_of_replicas":1,"number_of_shards":3}}}"#;
    let es_info = es_info.replace("logs", &name);
    Ok(HttpResponse::Ok()
        .content_type(http::header::ContentType::json())
        .insert_header(("X-Elastic-Product", "Elasticsearch"))
        .body(es_info))
}

#[put("/{org_id}/_index_template/{name}")]
async fn org_index_template_create(
    path: web::Path<(String, String)>,
    _body: web::Bytes,
) -> Result<HttpResponse, Error> {
    let (_org_id, name) = path.into_inner();
    let es_info = r#"{"name":"logs","message":"ok"}"#;
    let es_info = es_info.replace("logs", &name);
    Ok(HttpResponse::Ok()
        .content_type(http::header::ContentType::json())
        .insert_header(("X-Elastic-Product", "Elasticsearch"))
        .body(es_info))
}

#[route("/{org_id}/_data_stream/{name}", method = "GET", method = "HEAD")]
async fn org_data_stream(path: web::Path<(String, String)>) -> Result<HttpResponse, Error> {
    let (_org_id, name) = path.into_inner();
    let es_info = r#"{"data_streams":{"name":"logs","timestamp_field":{"name":"_timestamp"}}}"#;
    let es_info = es_info.replace("logs", &name);
    Ok(HttpResponse::Ok()
        .content_type(http::header::ContentType::json())
        .insert_header(("X-Elastic-Product", "Elasticsearch"))
        .body(es_info))
}

#[put("/{org_id}/_data_stream/{name}")]
async fn org_data_stream_create(
    path: web::Path<(String, String)>,
    _body: web::Bytes,
) -> Result<HttpResponse, Error> {
    let (_org_id, name) = path.into_inner();
    let es_info = r#"{"name":"logs","message":"ok"}"#;
    let es_info = es_info.replace("logs", &name);
    Ok(HttpResponse::Ok()
        .content_type(http::header::ContentType::json())
        .insert_header(("X-Elastic-Product", "Elasticsearch"))
        .body(es_info))
}
