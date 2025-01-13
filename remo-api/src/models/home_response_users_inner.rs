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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct HomeResponseUsersInner {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "joined_at", skip_serializing_if = "Option::is_none")]
    pub joined_at: Option<String>,
    #[serde(rename = "location_state", skip_serializing_if = "Option::is_none")]
    pub location_state: Option<String>,
    #[serde(rename = "nickname", skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,
    #[serde(rename = "role", skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
}

impl HomeResponseUsersInner {
    pub fn new() -> HomeResponseUsersInner {
        HomeResponseUsersInner {
            id: None,
            joined_at: None,
            location_state: None,
            nickname: None,
            role: None,
        }
    }
}

