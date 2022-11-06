#![allow(dead_code, unused_imports)]

#[macro_use]
extern crate rocket;

mod dns;
mod docker;

use crate::dns::agent_dns_lookup;
use crate::docker::data::{Info, MyContainerSummary, NodeInfo};
use crate::docker::Docker;

use rocket::fs::NamedFile;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use std::env;
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};

static AGENT_PORT: RwLock<i32> = RwLock::new(9511);

async fn get_request<T>(url: String) -> Result<T, String>
where
    T: for<'a> Deserialize<'a> + Serialize,
{
    let res: T = reqwest::get(url)
        .await
        .map_err(|e| e.to_string())?
        .json()
        .await
        .map_err(|e| e.to_string())?;
    Ok(res)
}

#[get("/nodes")]
async fn manager_nodes() -> Result<Json<Vec<NodeInfo>>, String> {
    match Docker::get_nodes_info().await {
        Err(e) => Err(e),
        Ok(nodes) => Ok(Json(nodes)),
    }
}

#[get("/agents/dns")]
async fn manager_dns() -> Json<Vec<String>> {
    let ips = agent_dns_lookup();
    Json(ips)
}

#[get("/<ip>/info")]
async fn manager_info(ip: &str) -> Result<Json<Info>, String> {
    match get_request::<Info>(format!(
        "http://{}:{}/api/dev/info",
        ip,
        *AGENT_PORT.read().unwrap()
    ))
    .await
    {
        Ok(res) => Ok(Json(res)),
        Err(e) => Err(e),
    }
}

#[get("/info")]
async fn agent_info() -> Json<Info> {
    let info = Docker::get_node_info().await;
    Json(info)
}

#[get("/<ip>/containers")]
async fn manager_containers(ip: &str) -> Result<Json<Vec<MyContainerSummary>>, String> {
    match get_request::<Vec<MyContainerSummary>>(format!(
        "http://{}:{}/api/dev/containers",
        ip,
        *AGENT_PORT.read().unwrap()
    ))
    .await
    {
        Ok(res) => Ok(Json(res)),
        Err(e) => Err(e),
    }
}

#[get("/containers")]
async fn agent_containers() -> Json<Vec<MyContainerSummary>> {
    let containers = Docker::get_containers().await;
    Json(containers)
}

#[get("/")]
async fn index() -> Option<NamedFile> {
    NamedFile::open("www/index.html").await.ok()
}

#[get("/healthcheck")]
fn healthcheck() -> &'static str {
    "OK"
}

#[get("/<file..>")]
async fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("www/").join(file)).await.ok()
}

#[launch]
fn rocket() -> _ {
    let mut port = 3500; // local development
    match env::var("VISUALIZER_TYPE").unwrap_or_default().as_str() {
        "manager" => port = 9510,
        "agent" => port = 9511,
        _ => {}
    }
    if port == 3500 {
        *AGENT_PORT.write().unwrap() = 3500;
    }

    log::info!("starting on port {}", port);

    let figment = rocket::Config::figment()
        .merge(("port", port))
        .merge(("address", "0.0.0.0"));

    rocket::custom(figment)
        .mount(
            "/api/dev",
            routes![
                agent_containers,
                agent_info,
                manager_containers,
                manager_dns,
                manager_info,
                manager_nodes,
            ],
        )
        .mount("/", routes![index, healthcheck, files])
}
