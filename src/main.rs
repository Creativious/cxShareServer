use std::env;
use actix_web::{middleware, App, HttpServer};
use crate::config::Config;

mod server;
mod config;
mod auth;
mod database;

const CONFIG_LOCATION_LINUX: &str = "/etc/cxshare/cxshare.conf";
const CONFIG_LOCATION_WINDOWS: &str = "%APPDATA%\\cxshare\\cxshare.conf";

const DEFAULT_ADDRESS: &str = "0.0.0.0";
const DEFAULT_PORT: u16 = 9090;

#[actix_web::main]
async fn main() {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();
    
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
        // then register http requests handler

    })
        .bind("0.0.0.0:9090").unwrap()
        .run()
        .await.expect("TODO: panic message");
}

async fn handle_startup() {
    let config: Config;
    if is_windows().await {
        config = get_windows_config().await.unwrap_or(Config::new(DEFAULT_ADDRESS.to_string(), DEFAULT_PORT))
    }
    else {
        // Linux is the default
        config = get_linux_config().await.unwrap_or(Config::new(DEFAULT_ADDRESS.to_string(), DEFAULT_PORT))
    }
}

async fn get_linux_config() -> Option<Config> {
    Config::from_path(CONFIG_LOCATION_LINUX)
}

async fn get_windows_config() -> Option<Config> {
    Config::from_path(CONFIG_LOCATION_WINDOWS)
}

async fn is_linux() -> bool {
    cfg!(target_os = "linux")
}

async fn is_windows() -> bool {
    cfg!(target_os = "windows")
}