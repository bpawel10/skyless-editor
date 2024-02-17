// TODO: remove these allows
#![allow(dead_code)]
#![allow(unused)]

use crate::parse::{dat, otb, otbm, spr, Parse};
use crate::transport::websocket::WebSocket;
use crate::{
    detect::Detect,
    load::{Error, Load},
    project::Project,
    skyless::SkylessProject,
    tfs::TfsProject,
};
use futures::StreamExt;
use model::World;
use serde::Serialize;
use std::fs::File;
use std::{
    net::{Ipv4Addr, SocketAddr, SocketAddrV4},
    path::PathBuf,
    sync::Arc,
};
use tauri::{AppHandle, Manager};
use tokio::net::TcpListener;

pub mod detect;
pub mod load;
pub mod parse;
pub mod transport;

pub mod project;
pub mod skyless;
pub mod tfs;

#[tauri::command]
fn get_websocket_url(app: AppHandle) -> SocketAddr {
    app.state::<Arc<WebSocket>>().url() // FIXME: it can panic, use try_state instead
}

#[tauri::command]
fn detect(directory: PathBuf) -> Option<Project> {
    SkylessProject::detect(&directory)
        .map(From::from)
        .or_else(|| TfsProject::detect(&directory).map(From::from))
}

#[tauri::command]
async fn load(app: AppHandle, project: TfsProject) -> Result<(), Error> {
    let transport = app.state::<Arc<WebSocket>>().inner();
    project.load(transport.clone()).await?; // FIXME: it can panic, use try_state instead
    Ok(())
}

fn main() {
    tauri::Builder::default()
        .plugin(WebSocket::init())
        .invoke_handler(tauri::generate_handler![get_websocket_url, detect, load,])
        .run(tauri::generate_context!())
        .expect("Error while running Tauri application");
}
