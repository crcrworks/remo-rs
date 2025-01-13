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

/// DeviceResponseUsersInner : Deprecated. Do not use in new code.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceResponseUsersInner {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "nickname", skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,
    #[serde(rename = "superuser", skip_serializing_if = "Option::is_none")]
    pub superuser: Option<bool>,
}

impl DeviceResponseUsersInner {
    /// Deprecated. Do not use in new code.
    pub fn new() -> DeviceResponseUsersInner {
        DeviceResponseUsersInner {
            id: None,
            nickname: None,
            superuser: None,
        }
    }
}

