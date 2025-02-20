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
pub struct ApplianceResponseQrioLock {
    #[serde(rename = "bd_address", skip_serializing_if = "Option::is_none")]
    pub bd_address: Option<String>,
    #[serde(rename = "device", skip_serializing_if = "Option::is_none")]
    pub device: Option<Box<models::ApplianceResponseQrioLockDevice>>,
    #[serde(rename = "is_available", skip_serializing_if = "Option::is_none")]
    pub is_available: Option<bool>,
    #[serde(rename = "sub_bd_address", skip_serializing_if = "Option::is_none")]
    pub sub_bd_address: Option<String>,
}

impl ApplianceResponseQrioLock {
    pub fn new() -> ApplianceResponseQrioLock {
        ApplianceResponseQrioLock {
            bd_address: None,
            device: None,
            is_available: None,
            sub_bd_address: None,
        }
    }
}

