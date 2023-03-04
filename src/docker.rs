pub mod data;
use crate::docker::data::{stats::ContainerStats, Info, MyContainerSummary, NodeInfo};

use docker_api::models::{ContainerSummary, Node, NodeStatus};
use docker_api::opts::ContainerListOpts;
use docker_api::Docker as DockerAPI;
use rocket::futures::StreamExt;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, error::Error, vec};
use sysinfo::{CpuExt, DiskExt, NetworkExt, NetworksExt, ProcessExt, System, SystemExt};

use self::data::Disk;

pub struct Docker;
impl Docker {
    pub fn get_docker_api() -> DockerAPI {
        // docker api v1.41
        let api_version = docker_api::ApiVersion::new(1, Some(41), None);

        #[cfg(unix)]
        let docker =
            docker_api::Docker::new_versioned("unix://var/run/docker.sock", api_version).unwrap();
        #[cfg(not(unix))]
        let docker =
            docker_api::Docker::new_versioned("tcp://localhost:2375", api_version).unwrap();

        // println!("endpoint: {}", api_version.make_endpoint(&String::new()));

        docker
    }

    pub async fn get_container_stats(api: &DockerAPI, id: String) -> ContainerStats {
        let value = api
            .containers()
            .get(&id)
            .stats()
            .next()
            .await
            .unwrap()
            .unwrap();
        serde_json::from_value::<ContainerStats>(value).unwrap()
    }

    pub async fn get_containers() -> Vec<MyContainerSummary> {
        let docker = Docker::get_docker_api();
        let containers = docker.containers().list(&Default::default()).await.unwrap();

        let mut list: Vec<MyContainerSummary> = vec![];

        for c in containers {
            let stats = Docker::get_container_stats(&docker, c.id.clone().unwrap()).await;
            let element = MyContainerSummary {
                id: c.id.unwrap(),
                image: c.image.unwrap(),
                labels: c.labels.unwrap(),
                names: c.names.unwrap(),
                state: c.state.unwrap(),
                status: c.status.unwrap(),
                stats,
            };
            list.push(element);
        }

        list
    }

    pub async fn get_node_info() -> Info {
        let mut sys = System::new_all();
        sys.refresh_all();

        let docker = Docker::get_docker_api();
        let info = docker.info().await.unwrap();
        let swarm = info.swarm.clone().unwrap();

        let to_go = 1024 * 1024 * 1024;
        let disk_one = sys.disks().get(0).unwrap();
        let total_space = u32::try_from(disk_one.total_space() / to_go).unwrap();
        let available_space = u32::try_from(disk_one.available_space() / to_go).unwrap();
        let disk_percent = f64::from(available_space) / f64::from(total_space);
        // log::info!("disk: {:?}", d.total_space());
        // log::info!("disk: {:?}", d.available_space());

        // memory usage
        let mem_usage = u16::try_from(sys.used_memory() / 1024 / 1024).unwrap();
        let mem_total = u16::try_from(sys.total_memory() / 1024 / 1024).unwrap();
        let mem_percent = f64::from(mem_usage) / f64::from(mem_total);

        // get cpu info
        let cpu_count = sys.cpus().len();
        sys.refresh_cpu();
        for cpu in sys.cpus() {
            cpu.cpu_usage();
        }
        std::thread::sleep(std::time::Duration::from_millis(1000));
        sys.refresh_cpu();
        let mut usage = 0f32;
        for cpu in sys.cpus() {
            usage += cpu.cpu_usage();
            // println!("{}% ", cpu.cpu_usage());
        }
        usage /= cpu_count as f32;

        let disk = Disk {
            total_space,
            available_space,
            disk_percent: 1f64 - disk_percent,
        };

        Info {
            node_id: swarm.node_id.unwrap(),
            node_address: swarm.node_addr.unwrap(),
            ncpu: info.ncpu.unwrap(),
            mem_total: sys.total_memory(),
            operating_system: info.operating_system.unwrap(),
            cpu_count,
            cpu_usage: usage,
            mem_usage: mem_percent,
            disk,
        }
    }

    pub async fn get_nodes_info() -> Result<Vec<NodeInfo>, String> {
        // get nodes
        let nodes = match Docker::get_nodes().await {
            Ok(nodes) => nodes,
            Err(e) => return Err(e),
        };

        let mut vec: Vec<NodeInfo> = vec![];

        // get individual node by ip
        for node in nodes {
            let status = node.status.unwrap();
            let state = status.state.clone().unwrap();
            let spec = node.spec.clone().unwrap();
            let addr = status.addr.clone().unwrap();

            let node_info = NodeInfo {
                id: node.id.unwrap(),
                version: node.version.unwrap().index.unwrap(),
                address: addr,
                role: spec.role.unwrap(),
                availability: spec.availability.unwrap(),
                state: state.clone(),
            };
            vec.push(node_info);

            // println!("{:?}", status);
            // log::info!("{:?}", status);
        }

        Ok(vec)
    }

    pub async fn get_nodes() -> Result<Vec<Node>, String> {
        let docker = Docker::get_docker_api();

        let Ok(nodes) = docker.nodes().list(&Default::default()).await else {
           return Err(String::from("failed to get nodes"))
        };

        Ok(nodes)
    }
}
