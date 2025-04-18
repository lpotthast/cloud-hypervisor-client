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
pub struct VmCoredumpData {
    #[serde(rename = "destination_url", skip_serializing_if = "Option::is_none")]
    pub destination_url: Option<String>,
}

impl VmCoredumpData {
    pub fn new() -> VmCoredumpData {
        VmCoredumpData {
            destination_url: None,
        }
    }
}
