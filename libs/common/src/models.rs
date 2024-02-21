use log::error;
use serde_derive::Serialize;
use crate::biz_code::BizCode;
use crate::biz_error::BizError;

#[derive(Serialize, Clone)]
pub struct LogBody {
    server_name: String,
    server_ip: String,
    log_info: String,
}

impl LogBody {
    pub fn new(server_name: String, server_ip: String, log_info: String) -> Self {
        Self {
            server_name,
            server_ip,
            log_info,
        }
    }

    pub fn to_json(&self) -> Result<String, BizError> {
        serde_json::to_string(self).map_err(|err| {
            error!("LogBody to json error: {:?}", err);
            BizError::new(BizCode::LOG_TO_JSON_STRING_ERROR)
        })
    }
}