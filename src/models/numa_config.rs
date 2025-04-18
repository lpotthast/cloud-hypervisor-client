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
pub struct NumaConfig {
    #[serde(rename = "guest_numa_id")]
    pub guest_numa_id: i32,
    #[serde(rename = "cpus", skip_serializing_if = "Option::is_none")]
    pub cpus: Option<Vec<i32>>,
    #[serde(rename = "distances", skip_serializing_if = "Option::is_none")]
    pub distances: Option<Vec<models::NumaDistance>>,
    #[serde(rename = "memory_zones", skip_serializing_if = "Option::is_none")]
    pub memory_zones: Option<Vec<String>>,
    #[serde(rename = "sgx_epc_sections", skip_serializing_if = "Option::is_none")]
    pub sgx_epc_sections: Option<Vec<String>>,
    #[serde(rename = "pci_segments", skip_serializing_if = "Option::is_none")]
    pub pci_segments: Option<Vec<i32>>,
}

impl NumaConfig {
    pub fn new(guest_numa_id: i32) -> NumaConfig {
        NumaConfig {
            guest_numa_id,
            cpus: None,
            distances: None,
            memory_zones: None,
            sgx_epc_sections: None,
            pci_segments: None,
        }
    }
}
