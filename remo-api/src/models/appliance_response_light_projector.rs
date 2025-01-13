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
pub struct ApplianceResponseLightProjector {
    #[serde(rename = "layout", skip_serializing_if = "Option::is_none")]
    pub layout: Option<Box<models::ApplianceResponseLightProjectorLayout>>,
}

impl ApplianceResponseLightProjector {
    pub fn new() -> ApplianceResponseLightProjector {
        ApplianceResponseLightProjector {
            layout: None,
        }
    }
}

