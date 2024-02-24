use std::path::PathBuf;

use ntex::web;
use ntex::web::{App, HttpServer, ServiceConfig};

use common::log4rs_config::ConfigLog4rs;
use common::log::info;
use ntex_auth::auth::auth_api;
use ntex_auth::middleware::auth_filter;

use crate::publish::settings::Settings;
use crate::publish::setup::init;

mod log_monitor;
mod publish;

#[ntex::main]
async fn main() -> std::io::Result<()> {
    let config = Settings::new().expect("读取配置文件出错");
    let log_path = PathBuf::from(config.store_path().clone());
    ConfigLog4rs::new(&log_path).unwrap().init_config().unwrap();

    init(config.remote_server).await;
    log_monitor::init_monitor(config.log_infos).await;

    let bind = "0.0.0.0:7201";
    info!("Starting server at: {}", &bind);

    HttpServer::new(move || {
        App::new().wrap(auth_filter::JwtFilter).configure(ser_config)
    }).bind(&bind)?.run().await
}

/// api入口
pub fn ser_config(cfg: &mut ServiceConfig) {
    cfg.service(web::scope("/api").service((auth_api::login,)));
}