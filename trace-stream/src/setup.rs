use std::path::PathBuf;
use tokio::sync::OnceCell;
use common::log4rs_config::ConfigLog4rs;
use ntex_auth::init_static_oauth;
use crate::log_monitor;
use crate::publish::api_upload::login;
use crate::settings::{RemoteServerConfig, Settings};


pub static REMOTE_SERVER: OnceCell<RemoteServerConfig> = OnceCell::const_new();


pub async fn init() {
    let config = Settings::new().expect("读取配置文件出错");
    let log_path = PathBuf::from(config.store_path().clone());
    ConfigLog4rs::new(&log_path).unwrap().init_config().unwrap();

    init_static_oauth(&config.static_oauth).await;
    store_remote_server_config(config.remote_server.to_owned()).await;
    login().await;

    // log monitors
    log_monitor::init_monitor(config.log_infos).await;
}

async fn store_remote_server_config(config: RemoteServerConfig) {
    REMOTE_SERVER.get_or_init(|| async { config }).await;
}