use std::path::PathBuf;
use common::log4rs_config::ConfigLog4rs;
use ntex_auth::init_static_oauth;
use crate::settings::Settings;
use crate::trace::file_store;

pub async fn init() {
    let config = Settings::new().expect("读取配置文件出错");
    let log_path = PathBuf::from(config.store_path().clone());

    file_store::init_store_path(&log_path).await;
    ConfigLog4rs::new(&log_path).unwrap().init_config().unwrap();
    init_static_oauth(&config.static_oauth).await;
}