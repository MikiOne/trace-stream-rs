use std::env;

use config::{Config, ConfigError, File};
use serde_derive::Deserialize;
use common::models::LogInfo;
use ntex_auth::StaticOauth;


#[derive(Debug, Clone, Deserialize)]
#[allow(unused)]
pub struct KafkaConfig {
    broker: String,
    topic: String,
    username: String,
    password: String,
}

impl KafkaConfig {
    pub fn get_broker(&self) -> &String { &self.broker }
    pub fn get_topic(&self) -> &String { &self.topic }

    pub fn get_username(&self) -> &String { &self.username }
    pub fn get_password(&self) -> &String { &self.password }
}

#[derive(Debug, Clone, Deserialize)]
#[allow(unused)]
pub struct RemoteServerConfig {
    server_domain: String,
    upload_uri: String,
    auth_uri: String,
}

impl RemoteServerConfig {
    pub fn get_server_domain(&self) -> &String {
        &self.server_domain
    }

    pub fn get_upload_uri(&self) -> String {
        self.with_domain(&self.upload_uri)
    }

    pub fn get_auth_uri(&self) -> String {
        self.with_domain(&self.auth_uri)
    }

    fn with_domain(&self, uri: &String) -> String {
        if uri.starts_with("/") {
            format!("{}{}", self.server_domain, uri)
        } else {
            format!("{}/{}", self.server_domain, uri)
        }
    }
}


#[derive(Debug, Clone, Deserialize)]
#[allow(unused)]
pub struct Settings {
    debug: bool,
    store_path: String,
    pub log_infos: Vec<LogInfo>,
    pub kafka_config: KafkaConfig,
    pub remote_server: RemoteServerConfig,

    pub static_oauth: StaticOauth,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "dev".into());
        let config_path = env::var("CONFIG_PATH").expect("请指定配置文件路径");
        let path = format!("{}-{}", config_path, run_mode);

        let config = File::with_name(&path).required(false);
        let setting = Config::builder().add_source(config).build()?;

        setting.try_deserialize()
    }

    pub fn is_debug(&self) -> bool {
        self.debug
    }
    pub fn store_path(&self) -> &String {
        &self.store_path
    }
}
