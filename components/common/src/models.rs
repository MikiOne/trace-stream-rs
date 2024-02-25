use std::sync::Arc;
use log::error;
use serde_derive::{Deserialize, Serialize};
use crate::biz_code::BizCode;
use crate::biz_error::BizError;
use crate::data_utils;

#[derive(Debug, Clone, Deserialize)]
#[allow(unused)]
pub struct LogInfo {
    project_name: String,
    server_name: String,
    path: String,
}

impl LogInfo {
    pub fn get_path(&self) -> &String {
        &self.path
    }

    pub fn get_server_name(&self) -> &String {
        &self.server_name
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogBody {
    pub project_name: String,
    pub server_name: String,
    pub server_ip: String,
    pub log_info: String,
    pub log_day: String,
}

impl LogBody {
    pub fn new(arc_log: Arc<LogInfo>, server_ip: String, log_info: String) -> Self {
        Self {
            project_name: arc_log.project_name.to_owned(),
            server_name: arc_log.server_name.to_owned(),
            server_ip,
            log_info,
            log_day: data_utils::to_day_str()
        }
    }

    pub fn to_json(&self) -> Result<String, BizError> {
        serde_json::to_string(self).map_err(|err| {
            error!("LogBody to json error: {:?}", err);
            BizError::new(BizCode::LOG_TO_JSON_STRING_ERROR)
        })
    }

    pub fn print_base(&self) -> String {
        format!("day[{}] server[{}:{}]", self.log_day, self.server_ip, self.server_name)
    }
}