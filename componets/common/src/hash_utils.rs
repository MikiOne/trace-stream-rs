use bcrypt::{hash, DEFAULT_COST, verify};
use log::error;
use crate::biz_code::BizCode;
use crate::biz_error::BizError;


pub fn md5_hex(input: &str) -> String {
    format!("{:x}", md5::compute(input))
}

pub fn bcrypt_hash(password: &str) -> Result<String, BizError> {
    hash(password, DEFAULT_COST).map_err(|err| {
        error!("pwd[{}] bcrypt hash error: {:?}", password, err);
        BizError::with_err(BizCode::BCRYPT_ERROR, err.to_string())
    })
}

pub fn bcrypt_verify(password: &str, hash: &str) -> Result<bool, BizError> {
    verify(password, hash).map_err(|err| {
        error!("pwd[{}] hash[{}] bcrypt verify error: {}", password, hash, err);
        BizError::with_err(BizCode::BCRYPT_ERROR, err.to_string())
    })
}



#[test]
fn test_md5() {
    let data = "sdtL.TraceSer-Ver-2021-08-01T00:00:00Z";
    let digest = md5_hex(data);
    println!("MD5 之后的结果: {}", digest);
}


#[test]
fn test_bcrypt_hash() {
    let val = bcrypt_hash("4cb8304e3eab0d86044fcf9dd07f7cd5").unwrap();
    println!("bcrypt hash: {}", val);
}

#[test]
fn test_hash() {
    let password = "mysecretpassword";

    // 对密码进行加密，使用默认的计算成本
    match hash(password, DEFAULT_COST) {
        Ok(hashed_password) => {
            println!("加密后的密码是：{}", hashed_password);
        }
        Err(e) => {
            println!("密码加密时出错：{}", e);
        }
    }
}


#[test]
fn can_verify_hash_generated_from_some_online_tool() {
    let hash = "$2a$04$UuTkLRZZ6QofpDOlMz32MuuxEHA43WOemOYHPz6.SjsVsyO1tDU96";
    assert!(verify("password", hash).unwrap());
}