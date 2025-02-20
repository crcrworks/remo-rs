/*
 * Nature API
 *
 * Read/Write Nature Remo
 *
 * The version of the OpenAPI document: 2.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

use serde_with::serde_as;

#[serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplianceResponseBleSesame {
    #[serde(rename = "device_type", skip_serializing_if = "Option::is_none")]
    pub device_type: Option<String>,
    #[serde(rename = "key_level", skip_serializing_if = "Option::is_none")]
    pub key_level: Option<String>,
    #[serde_as(as = "Option<serde_with::base64::Base64>")]
    #[serde(rename = "user_index", skip_serializing_if = "Option::is_none")]
    pub user_index: Option<Vec<u8>>,
    #[serde(rename = "uuid", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub uuid: Option<Option<serde_json::Value>>,
}

impl ApplianceResponseBleSesame {
    pub fn new() -> ApplianceResponseBleSesame {
        ApplianceResponseBleSesame {
            device_type: None,
            key_level: None,
            user_index: None,
            uuid: None,
        }
    }
}

