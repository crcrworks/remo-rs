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
pub struct ApplianceResponseLight {
    #[serde(rename = "buttons", skip_serializing_if = "Option::is_none")]
    pub buttons: Option<Vec<models::ApplianceResponseLightButtonsInner>>,
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<Box<models::ApplianceResponseLightState>>,
}

impl ApplianceResponseLight {
    pub fn new() -> ApplianceResponseLight {
        ApplianceResponseLight {
            buttons: None,
            state: None,
        }
    }
}

