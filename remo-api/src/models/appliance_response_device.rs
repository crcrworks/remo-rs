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
pub struct ApplianceResponseDevice {
    #[serde(rename = "bt_mac_address", skip_serializing_if = "Option::is_none")]
    pub bt_mac_address: Option<String>,
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "firmware_version", skip_serializing_if = "Option::is_none")]
    pub firmware_version: Option<String>,
    #[serde(rename = "humidity_offset", skip_serializing_if = "Option::is_none")]
    pub humidity_offset: Option<f32>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "mac_address", skip_serializing_if = "Option::is_none")]
    pub mac_address: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "serial_number", skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    #[serde(rename = "temperature_offset", skip_serializing_if = "Option::is_none")]
    pub temperature_offset: Option<f32>,
    #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

impl ApplianceResponseDevice {
    pub fn new() -> ApplianceResponseDevice {
        ApplianceResponseDevice {
            bt_mac_address: None,
            created_at: None,
            firmware_version: None,
            humidity_offset: None,
            id: None,
            mac_address: None,
            name: None,
            serial_number: None,
            temperature_offset: None,
            updated_at: None,
        }
    }
}

