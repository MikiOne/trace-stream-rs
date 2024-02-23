use std::path::PathBuf;

use log::info;
use ntex::web::{App, HttpServer};
use ntex::web;
use ntex::web::{Responder, ServiceConfig};

use crate::auth::auth_api;
use crate::log4rs_config::ConfigLog4rs;
use crate::trace::store_compress::init_store_path;
use crate::trace::trace_api;
use crate::middleware::auth_filter;
use crate::settings::Settings;

mod log4rs_config;
mod settings;
mod auth;
mod middleware;
mod trace;

#[ntex::main]
async fn main() -> std::io::Result<()> {
    let settings = Settings::new().expect("读取配置文件出错");
    let store_path = settings.store_path();
    let log_path = PathBuf::from(store_path.clone());

    init_store_path(&log_path).await;
    ConfigLog4rs::new(&log_path).unwrap().init_config().unwrap();

    let bind = "0.0.0.0:7200";
    info!("Starting server at: {}", &bind);

    HttpServer::new(move || {
        App::new().wrap(auth_filter::JwtFilter).configure(ser_config)
    }).bind(&bind)?.run().await
}

/// api入口
pub fn ser_config(cfg: &mut ServiceConfig) {
    cfg.service(web::scope("/api").service((auth_api::login, trace_api::store_log)));
}
