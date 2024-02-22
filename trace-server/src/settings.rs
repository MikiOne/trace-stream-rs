use std::env;

use config::{Config, ConfigError, File};
use serde_derive::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[allow(unused)]
pub struct Settings {
    debug: bool,
    store_path: String,
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
