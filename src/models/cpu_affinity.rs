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
pub struct CpuAffinity {
    #[serde(rename = "vcpu")]
    pub vcpu: i32,
    #[serde(rename = "host_cpus")]
    pub host_cpus: Vec<i32>,
}

impl CpuAffinity {
    pub fn new(vcpu: i32, host_cpus: Vec<i32>) -> CpuAffinity {
        CpuAffinity { vcpu, host_cpus }
    }
}
