/*
 * Cloud Hypervisor API
 *
 * Local HTTP based API for managing and inspecting a cloud-hypervisor virtual machine.
 *
 * The version of the OpenAPI document: 0.3.0
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TpmConfig {
    #[serde(rename = "socket")]
    pub socket: String,
}

impl TpmConfig {
    pub fn new(socket: String) -> TpmConfig {
        TpmConfig { socket }
    }
}
