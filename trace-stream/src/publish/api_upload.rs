use std::path::PathBuf;
use reqwest::{header, Response, StatusCode};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use tokio::sync::OnceCell;

use common::biz_code::BizCode;
use common::biz_error::BizError;
use common::biz_resp::{Empty, RespData};
use common::log::{debug, error, info, warn};
use common::models::LogBody;
use ntex_auth::auth::models::TokenInfo;
use crate::publish::http_client::ReqwestClient;
use crate::settings::RemoteServerConfig;
use crate::setup::REMOTE_SERVER;

// todo: 这里不能使用一次初始化的方式，因为登录后会更新token
static ACCESS_TOKEN: OnceCell<String> = OnceCell::const_new();

pub async fn store_access_token(token: &String) {
    ACCESS_TOKEN.get_or_init(|| async { token.to_owned() }).await;
}

pub async fn login() {
    let login_uri = match REMOTE_SERVER.get() {
        Some(remote_server) => remote_server.get_auth_uri(),
        None => {
            error!("Login to trace-server error: RemoteServerConfig not found.");
            return;
        }
    };

    let client = ReqwestClient::build();
    let resp = client
        .post(login_uri)
        .header(header::CONTENT_TYPE, "application/json")
        .body(r#"{
            "uid": "101",
            "pwd_md5": "4cb8304e3eab0d86044fcf9dd07f7cd5"
        }"#)
        .send()
        .await.map_err(|err| {
        error!("Login to trace-server error: {:?}", err);
    }).unwrap();

    let resp_data = resp.json::<RespData<TokenInfo>>().await;
    info!("login resp_data: {:?}", resp_data);

    let store_token = |resp: RespData<TokenInfo>| async {
        if let Some(token) = resp.data {
            let access_token = format!("Bearer {}", token.access_token);
            store_access_token(&access_token).await;
        }
    };

    match resp_data {
        Ok(resp) => {
            store_token(resp).await;
        }
        Err(errs) => {
            error!("Parse response to RespData error: {:?}", errs);
        }
    }
}

fn build_bearer_headers() -> Result<HeaderMap, BizError> {
    let mut headers = HeaderMap::new();
    if let Some(token) = ACCESS_TOKEN.get() {
        headers.insert(
            header::AUTHORIZATION,
            HeaderValue::from_static(token.as_str()),
        );
    }
    Ok(headers)
}


pub async fn send_log(ser_config: &RemoteServerConfig, log_body: &LogBody) -> Result<(), BizError> {
    let upload_uri = ser_config.get_upload_uri();
    let mut headers = build_bearer_headers()?;
    headers.insert(
        header::CONTENT_TYPE,
        // HeaderValue::from_static("application/octet-stream"),
        HeaderValue::from_static("application/json"),
    );
    debug!("send_log headers: {:?}", &headers);

    let msg_body = log_body.to_json()?;
    debug!("send_log msg: {:?}", &msg_body);

    let client = ReqwestClient::build();
    let resp = client
        .post(upload_uri)
        .headers(headers)
        .body(msg_body)
        .send()
        .await.map_err(|err| {
        error!("Send log to server error: {:?}", err);
        BizError::new(BizCode::REQWEST_ERROR)
    })?;

    parse_resp(resp).await;
    Ok(())
}


async fn parse_resp(resp: Response) {
    if resp.status() != StatusCode::OK {
        warn!("send_log response: {:?}", resp);
        return;
    }

    let resp_data = resp.json::<RespData<Empty>>().await;
    debug!("parse send_log Response, resp_data: {:?}", resp_data);

    match resp_data {
        Ok(resp) => {
            if resp.code.starts_with("AU") {
                login().await;
            }
        }
        Err(errs) => {
            error!("Parse response to RespData error: {:?}", errs);
        }
    }
}
