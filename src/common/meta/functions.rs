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

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use vrl::{
    compiler::{CompileConfig, Program, TimeZone, VrlRuntime},
    prelude::Function,
};

use super::StreamType;

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Transform {
    pub function: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub params: String,
    #[serde(default)]
    pub num_args: u8,
    #[serde(default = "default_trans_type")]
    pub trans_type: Option<u8>, // 0=vrl 1=lua
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub streams: Option<Vec<StreamOrder>>,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct StreamOrder {
    #[serde(default)]
    pub stream: String,
    #[serde(default)]
    pub order: u8,
    #[serde(default)]
    pub stream_type: StreamType,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct StreamTransform {
    #[serde(flatten)]
    pub transform: Transform,
    #[serde(default)]
    pub stream: String,
    #[serde(default)]
    pub order: u8,
    #[serde(default)]
    pub stream_type: StreamType,
}

impl PartialEq for StreamTransform {
    fn eq(&self, other: &Self) -> bool {
        self.stream == other.stream
            && self.transform.name == other.transform.name
            && self.stream_type == other.stream_type
    }
}

impl Transform {
    pub fn to_stream_transform(&self) -> Vec<StreamTransform> {
        let mut ret: Vec<StreamTransform> = vec![];
        if let Some(streams) = &self.streams {
            let mut func = self.clone();
            func.streams = None;
            for stream in streams {
                ret.push(StreamTransform {
                    transform: func.clone(),
                    stream: stream.stream.clone(),
                    order: stream.order,
                    stream_type: stream.stream_type,
                })
            }
        }
        ret
    }
}

impl PartialEq for Transform {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.function == other.function && self.params == other.params
    }
}
#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct ZoFunction<'a> {
    pub name: &'a str,
    pub text: &'a str,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct FunctionList {
    pub list: Vec<Transform>,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct StreamFunctionsList {
    pub list: Vec<StreamTransform>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VRLConfig {
    pub runtime: VrlRuntime,
    pub timezone: TimeZone,
}

fn default_trans_type() -> Option<u8> {
    Some(0)
}

pub struct VRLCompilerConfig {
    pub config: CompileConfig,
    pub functions: Vec<Box<dyn Function>>,
}

pub struct VRLRuntimeConfig {
    pub config: CompileConfig,
    pub program: Program,
    pub fields: Vec<String>,
}

pub struct VRLResultResolver {
    pub program: Program,
    pub fields: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::utils::json;

    #[test]
    fn test_functions() {
        let trans = Transform {
            function: "function jsconcat(a,b){return a+b}".to_string(),
            name: "jsconcat".to_string(),
            trans_type: Some(1),
            params: "row".to_string(),
            num_args: 1,
            streams: Some(vec![StreamOrder {
                stream: "test".to_string(),
                order: 1,
                stream_type: StreamType::Logs,
            }]),
        };

        let mod_trans = Transform {
            function: "function jsconcat(a,b){return a+b}".to_string(),
            name: "jsconcat".to_string(),
            trans_type: Some(1),
            params: "row".to_string(),
            num_args: 1,
            streams: None,
        };
        assert_eq!(trans, mod_trans);

        let trans_str = json::to_string(&trans).unwrap();
        let trans2: Transform = json::from_str(&trans_str).unwrap();
        assert_eq!(format!("{:?}", trans), format!("{:?}", trans2));

        let trans_list = FunctionList {
            list: vec![trans, trans2],
        };
        assert!(!trans_list.list.is_empty());
        let trans_list_str = json::to_string(&trans_list.clone()).unwrap();
        let trans_list2: FunctionList = json::from_str(&trans_list_str).unwrap();
        assert_eq!(trans_list.list.len(), trans_list2.list.len());
    }

    #[test]
    fn test_zo_function() {
        let f1 = ZoFunction {
            name: "test",
            text: "test",
        };
        let f1_str = json::to_string(&f1).unwrap();
        let f2: ZoFunction = json::from_str(&f1_str).unwrap();
        assert_eq!(f1.name, f2.name);
        assert_eq!(format!("{:?}", f1), format!("{:?}", f2));
    }
}
