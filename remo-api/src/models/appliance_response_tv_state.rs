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
pub struct ApplianceResponseTvState {
    #[serde(rename = "input", skip_serializing_if = "Option::is_none")]
    pub input: Option<String>,
}

impl ApplianceResponseTvState {
    pub fn new() -> ApplianceResponseTvState {
        ApplianceResponseTvState {
            input: None,
        }
    }
}

