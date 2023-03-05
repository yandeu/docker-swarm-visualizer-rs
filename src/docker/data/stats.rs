use serde::{Deserialize, Serialize};
use structstruck;

#[derive(Debug, Serialize, Deserialize)]
pub struct CpuUsage {
    pub total_usage: i64,
}

// https://docs.docker.com/engine/api/v1.42/#tag/Container/operation/ContainerStats
structstruck::strike! {
    #[strikethrough[derive(Debug, Serialize, Deserialize)]]
    pub struct ContainerStats {
        cpu_stats: struct {
            pub cpu_usage: CpuUsage,
            pub system_cpu_usage: Option<i64>,
            pub online_cpus: i64
        },
        precpu_stats: struct {
            pub cpu_usage: CpuUsage,
            pub system_cpu_usage: Option<i64>
        },
        memory_stats: struct  {
            pub limit: i64,
            pub usage: i64
        }
    }
}
