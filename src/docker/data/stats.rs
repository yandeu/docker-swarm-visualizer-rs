use serde::{Deserialize, Serialize};
use structstruck;

#[derive(Debug, Serialize, Deserialize)]
pub struct CpuUsage {
    pub total_usage: i64,
}

structstruck::strike! {
    #[strikethrough[derive(Debug, Serialize, Deserialize)]]
    pub struct ContainerStats {
        cpu_stats: struct {
            pub cpu_usage: CpuUsage,
            pub system_cpu_usage: i64,
        },
        precpu_stats: struct {
            pub cpu_usage: CpuUsage
        },
        memory_stats: struct  {
            pub limit: i64,
            pub usage: i64,
            pub max_usage: i64,
            // pub stats: struct  {
            //     pub total_inactive_file: i64,
            // }
        }
    }
}
