use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use self::stats::ContainerStats;

pub mod stats;

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeInfo {
    pub id: String,
    pub version: u64,
    pub address: String,
    pub role: String,
    pub availability: String,
    pub state: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Disk {
    pub total_space: u32,
    pub available_space: u32,
    pub disk_percent: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Info {
    pub node_id: String,
    pub node_address: String,
    pub ncpu: isize,
    pub mem_total: u64,
    pub operating_system: String,
    pub cpu_count: usize,
    pub cpu_usage: f32,
    pub mem_usage: f64,
    pub disk: Disk,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MyContainerSummary {
    pub id: String,
    pub image: String,
    pub labels: HashMap<String, String>,
    pub names: Vec<String>,
    pub state: String,
    pub status: String,
    pub stats: ContainerStats,
}
