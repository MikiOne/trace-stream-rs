use log::{error, info};
use ntex::web;
use ntex::web::{HttpResponse, Responder};
use ntex::web::types::Json;

use common::biz_code::BizCode;
use common::biz_resp::RespData;
use common::hash_utils::bcrypt_verify;

use crate::auth::auth_consts::{AUTH_UID, PWD_BCRYPT_HASH};
use crate::auth::jwt_handler;
use crate::auth::models::{LoginUser, Role};

#[web::post("/auth/token")]
pub async fn login(
    login_user: Json<LoginUser>,
) -> Result<impl Responder, web::Error> {
    info!("login user: {:?}", &login_user);

    if login_user.uid != AUTH_UID {
        return Ok(HttpResponse::Ok().json(&RespData::with_biz_code(BizCode::LOGIN_UID_ERR)));
    }
    let verify_result = bcrypt_verify(&login_user.pwd_md5, &PWD_BCRYPT_HASH);
    info!("bcrypt_verify result: {:?}", verify_result);

    let create_token = || {
        match jwt_handler::create_jwt(login_user.uid.to_owned(), &Role::User) {
            Ok(token) => {
                info!("login token info: {:?}", token);
                Ok(HttpResponse::Ok().json(&RespData::with_success(token)))
            }
            Err(err) => {
                error!("login error: {:?}", &err);
                Ok(HttpResponse::Ok().json(&RespData::from_biz_error(&err)))
            }
        }
    };

    match verify_result {
        Ok(val) => {
            if val { create_token() } else {
                Ok(HttpResponse::Ok().json(&RespData::with_biz_code(BizCode::LOGIN_PWD_ERR)))
            }
        }
        Err(err) => {
            error!("bcrypt_verify error: {:?}", &err);
            Ok(HttpResponse::Ok().json(&RespData::with_biz_code(BizCode::LOGIN_PWD_ERR)))
        }
    }
}