use std::path::PathBuf;

use ntex::web::{App, HttpServer};

use common::log4rs_config::ConfigLog4rs;
use common::log::info;
use log_upload::settings::Settings;
use ntex_auth::middleware::auth_filter;

mod log_monitor;

#[ntex::main]
async fn main() -> std::io::Result<()> {
    let config = Settings::new().expect("读取配置文件出错");
    let log_path = PathBuf::from(config.store_path().clone());
    ConfigLog4rs::new(&log_path).unwrap().init_config().unwrap();
    log_monitor::init_monitor(config.log_infos, config.remote_server).await;

    let bind = "0.0.0.0:7201";
    info!("Starting server at: {}", &bind);

    HttpServer::new(move || {
        App::new().wrap(auth_filter::JwtFilter)
    }).bind(&bind)?.run().await
}