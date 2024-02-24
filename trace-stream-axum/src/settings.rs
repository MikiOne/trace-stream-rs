// use std::env;
//
// use config::{Config, ConfigError, File};
// use serde_derive::Deserialize;
//
// #[derive(Debug, Clone, Deserialize)]
// #[allow(unused)]
// pub struct LogInfo {
//     path: String,
//     server_name: String,
// }
//
// impl LogInfo {
//     pub fn get_path(&self) -> &String {
//         &self.path
//     }
//
//     pub fn get_server_name(&self) -> &String {
//         &self.server_name
//     }
// }
//
// #[derive(Debug, Clone, Deserialize)]
// #[allow(unused)]
// pub struct KafkaConfig {
//     broker: String,
//     topic: String,
//     username: String,
//     password: String,
// }
//
// impl KafkaConfig {
//     pub fn get_broker(&self) -> &String { &self.broker }
//     pub fn get_topic(&self) -> &String { &self.topic }
//
//     pub fn get_username(&self) -> &String { &self.username }
//     pub fn get_password(&self) -> &String { &self.password }
// }
//
// #[derive(Debug, Clone, Deserialize)]
// #[allow(unused)]
// pub struct RemoteServerConfig {
//     server_domain: String,
//     upload_uri: String,
// }
//
// impl RemoteServerConfig {
//     pub fn get_server_domain(&self) -> &String { &self.server_domain }
//     pub fn get_upload_uri(&self) -> &String { &self.upload_uri }
// }
//
//
// #[derive(Debug, Clone, Deserialize)]
// #[allow(unused)]
// pub struct Settings {
//     debug: bool,
//     pub log_infos: Vec<LogInfo>,
//     pub kafka_config: KafkaConfig,
//     pub remote_server: RemoteServerConfig,
// }
//
// impl Settings {
//     pub fn new() -> Result<Self, ConfigError> {
//         let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "dev".into());
//         let config_path = env::var("CONFIG_PATH").expect("请指定配置文件路径");
//         let path = format!("{}-{}", config_path, run_mode);
//
//         let config = File::with_name(&path).required(false);
//         let setting = Config::builder().add_source(config).build()?;
//
//         setting.try_deserialize()
//     }
//
//     pub fn is_debug(&self) -> bool {
//         self.debug
//     }
// }
