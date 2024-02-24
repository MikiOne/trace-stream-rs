use serde_derive::{Deserialize, Serialize};

use crate::biz_code::BizCode;
use crate::biz_error::BizError;

pub type BizResult<T> = Result<T, BizError>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Empty {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RespData<T> {
    pub code: String,
    pub msg: String,
    pub data: Option<T>,
}

impl<T> RespData<T> {
    pub fn with_success(data: T) -> Self {
        let biz_code = BizCode::SUCCESS;
        let msg = biz_code.reason().unwrap().to_string();
        RespData { code: biz_code.code().to_string(), msg, data: Some(data) }
    }

    // pub fn success(data: T) -> HttpResponse
    // where
    //     T: serde::Serialize,
    // {
    //     let res_data = RespData::with_success(data);
    //     HttpResponse::Ok().json(&res_data)
    // }
}

impl RespData<()> {
    pub fn success() -> Self {
        let biz_code = BizCode::SUCCESS;
        let msg = biz_code.reason().unwrap().to_string();
        RespData { code: biz_code.code().to_string(), msg, data: None }
    }

    pub fn with_biz_code(biz_code: BizCode) -> Self {
        let msg = biz_code.reason().unwrap().to_string();
        RespData { code: biz_code.code().to_string(), msg, data: None }
    }

    pub fn with_biz_code_err(biz_code: BizCode, err: &String) -> Self {
        let msg = biz_code.reason().unwrap().to_string();
        let err_msg = format!("{}: {}", msg, err);
        RespData { code: biz_code.code().to_string(), msg: err_msg, data: None }
    }

    pub fn from_biz_error(biz_error: &BizError) -> Self {
            RespData { code: biz_error.biz_code.to_string(), msg: biz_error.to_string(), data: None }
    }
//
//     pub fn with_blocking_err(blocking_err: BlockingError<BizError>) -> HttpResponse {
//         match blocking_err {
//             BlockingError::Error(biz_error) => RespData::from_biz_error(&biz_error),
//             err => {
//                 error!("Web block error: {:?}", err);
//                 RespData::with_biz_code_err(BizCode::SYSTEM_ERROR, &err.to_string())
//             }
//         }
//     }
}
