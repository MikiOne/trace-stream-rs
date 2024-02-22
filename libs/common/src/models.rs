use log::error;
use serde_derive::{Deserialize, Serialize};
use crate::biz_code::BizCode;
use crate::biz_error::BizError;
use crate::data_utils;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogBody {
    pub server_name: String,
    pub server_ip: String,
    pub log_info: String,
    pub log_day: String,
}

impl LogBody {
    pub fn new(server_name: String, server_ip: String, log_info: String) -> Self {
        Self {
            server_name,
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

    pub fn display(&self) -> String {
        format!("[server:{}{}]: 「\n{}\n」", self.server_ip, self.server_name, self.log_info)
    }
}