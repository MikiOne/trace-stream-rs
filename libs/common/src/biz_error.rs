use std::fmt::{Debug, Display, Formatter};
use crate::biz_code::BizCode;

pub struct BizError {
    pub biz_code: BizCode,
    err_str: String,
}

impl BizError {
    pub fn new(biz_code: BizCode) -> Self {
        BizError { biz_code, err_str: "".to_string() }
    }
    pub fn with_err(biz_code: BizCode, err_str: String) -> Self {
        BizError { biz_code, err_str }
    }

    // pub fn with_err(biz_code: BizCode, err: Box<dyn std::error::Error>) -> Self {
    //     BizError { biz_code, err_str: Some(format!("{}", err)) }
    // }

    pub fn code_reason(&self) -> String {
        self.biz_code.code_reason()
    }
}

impl Display for BizError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}: {}", self.biz_code.reason().unwrap(), self.err_str)
    }
}

impl Debug for BizError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.code_reason(), self.err_str)
    }
}
