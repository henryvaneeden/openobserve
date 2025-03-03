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

use crate::common::utils::json;
use opentelemetry_proto::tonic::{
    common::v1::{any_value::Value, AnyValue},
    metrics::v1::{exemplar, number_data_point},
};

use super::get_value;

pub fn get_val(attr_val: &Option<AnyValue>) -> json::Value {
    match attr_val {
        Some(local_val) => match &local_val.value {
            Some(val) => match val {
                Value::StringValue(inner_val) => json::json!(inner_val.as_str()),
                Value::BoolValue(inner_val) => {
                    json::json!(inner_val.to_string())
                }
                Value::IntValue(inner_val) => {
                    json::json!(inner_val.to_string())
                }
                Value::DoubleValue(inner_val) => json::json!(inner_val.to_string()),
                Value::ArrayValue(inner_val) => {
                    let mut vals = vec![];
                    for item in inner_val.values.iter().cloned() {
                        vals.push(get_val(&Some(item)))
                    }
                    json::json!(vals)
                }
                Value::KvlistValue(inner_val) => {
                    let mut vals = json::Map::new();
                    for item in inner_val.values.iter().cloned() {
                        vals.insert(item.key, get_val(&item.value));
                    }
                    json::json!(vals)
                }
                Value::BytesValue(inner_val) => {
                    json::json!(inner_val)
                }
            },
            None => json::Value::Null,
        },
        None => json::Value::Null,
    }
}

pub fn get_severity_value(severity_number: i32) -> String {
    match severity_number {
        0 => "Unspecified",
        1 => "Trace",
        2 => "Trace2",
        3 => "Trace3",
        4 => "Trace4",
        5 => "Debug",
        6 => "Debug2",
        7 => "Debug3",
        8 => "Debug4",
        9 => "Info",
        10 => "Info2",
        11 => "Info3",
        12 => "Info4",
        13 => "Warn ",
        14 => "Warn2",
        15 => "Warn3",
        16 => "Warn4",
        17 => "Error",
        18 => "Error2",
        19 => "Error3",
        20 => "Error4",
        21 => "Fatal",
        22 => "Fatal2",
        23 => "Fatal3",
        24 => "Fatal4",
        _ => "Unspecified",
    }
    .into()
}

pub fn get_metric_val(attr_val: &Option<number_data_point::Value>) -> json::Value {
    match attr_val {
        Some(local_val) => match local_val {
            number_data_point::Value::AsDouble(val) => json::json!(val),
            number_data_point::Value::AsInt(val) => json::json!(*val as f64),
        },
        None => json::Value::Null,
    }
}

pub fn get_exemplar_val(attr_val: &Option<exemplar::Value>) -> json::Value {
    match attr_val {
        Some(local_val) => match local_val {
            exemplar::Value::AsDouble(val) => json::json!(val),
            exemplar::Value::AsInt(val) => json::json!(*val as f64),
        },
        None => json::Value::Null,
    }
}

pub fn get_val_for_attr(attr_val: json::Value) -> json::Value {
    let local_val = attr_val.as_object().unwrap();
    if let Some((_key, value)) = local_val.into_iter().next() {
        return serde_json::Value::String(get_value(value));
    };
    ().into()
}

pub fn get_val_with_type_retained(attr_val: &Option<AnyValue>) -> json::Value {
    match attr_val {
        Some(local_val) => match &local_val.value {
            Some(val) => match val {
                Value::StringValue(val) => {
                    json::json!(val)
                }
                Value::BoolValue(val) => {
                    json::json!(val)
                }
                Value::IntValue(val) => {
                    json::json!(val)
                }
                Value::DoubleValue(val) => {
                    json::json!(val)
                }
                Value::ArrayValue(val) => {
                    let mut vals = vec![];
                    for item in val.values.iter().cloned() {
                        vals.push(get_val(&Some(item)))
                    }
                    json::json!(vals)
                }
                Value::KvlistValue(val) => {
                    let mut vals = json::Map::new();
                    for item in val.values.iter().cloned() {
                        vals.insert(item.key, get_val(&item.value));
                    }
                    json::json!(vals)
                }
                Value::BytesValue(val) => {
                    json::json!(val)
                }
            },
            None => json::Value::Null,
        },
        None => json::Value::Null,
    }
}

#[cfg(test)]
mod tests {
    use opentelemetry_proto::tonic::common::v1::AnyValue;

    use super::*;

    #[test]
    fn test_get_val() {
        let in_str = "Test".to_string();
        let str_val = AnyValue {
            value: Some(Value::StringValue(in_str.clone())),
        };
        let resp = get_val(&Some(str_val));
        assert_eq!(resp.as_str().unwrap(), in_str);

        let in_bool = false;
        let bool_val = AnyValue {
            value: Some(Value::BoolValue(in_bool)),
        };
        let resp = get_val(&Some(bool_val));
        assert_eq!(resp.as_str().unwrap(), in_bool.to_string());

        let in_int = 20;
        let int_val = AnyValue {
            value: Some(Value::IntValue(in_int)),
        };
        let resp = get_val(&Some(int_val.clone()));
        assert_eq!(resp.as_str().unwrap(), in_int.to_string());

        let in_double = 20.00;
        let double_val = AnyValue {
            value: Some(Value::DoubleValue(in_double)),
        };
        let resp = get_val(&Some(double_val));
        assert_eq!(resp.as_str().unwrap(), in_double.to_string());

        let in_arr = vec![int_val.clone()];
        let arr_val = AnyValue {
            value: Some(Value::ArrayValue(
                opentelemetry_proto::tonic::common::v1::ArrayValue { values: in_arr },
            )),
        };
        let resp = get_val(&Some(arr_val));
        assert!(!resp.as_array().unwrap().is_empty());

        let kv_val = AnyValue {
            value: Some(Value::KvlistValue(
                opentelemetry_proto::tonic::common::v1::KeyValueList {
                    values: vec![opentelemetry_proto::tonic::common::v1::KeyValue {
                        key: in_str.clone(),
                        value: Some(int_val.clone()),
                    }],
                },
            )),
        };
        let resp = get_val(&Some(kv_val));
        assert!(resp.as_object().unwrap().contains_key(&in_str));

        let in_byte = Value::BytesValue(vec![8u8]);

        let byte_val = AnyValue {
            value: Some(in_byte),
        };
        let resp = get_val(&Some(byte_val));
        assert!(!resp.as_array().unwrap().is_empty());
    }
}
