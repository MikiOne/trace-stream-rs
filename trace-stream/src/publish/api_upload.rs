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
use crate::publish::settings::RemoteServerConfig;
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

// pub async fn send_file_and_data(&self, filepath: &PathBuf) -> Result<(), CommonError> {
//     info!("send_file_and_data filepath: {:?}", filepath);
//     let ser_url = SettingsCode::LogServerUrl.get_value().await;
//     let file_uri = SettingsCode::LogFileUri.get_value().await;
//
//     if let Some(request_id) = &self.msg {
//         let url = format!("{}{}{}", ser_url, file_uri, request_id);
//         let mut file = File::open(filepath).unwrap();
//         let mut file_content = vec![];
//         file.read_to_end(&mut file_content).unwrap();
//
//         let filename = filepath.file_name().unwrap().to_os_string().into_string().unwrap();
//         info!("send_file_and_data filename: {:?}", &filename);
//         let form = Form::new()
//             .part("file", Part::bytes(file_content).file_name(filename))
//             .part("data", Part::text(self.to_json()?));
//
//         let headers = build_logser_headers().await?;
//         let client = ReqwestClient::build();
//         let resp = client
//             .post(url)
//             .headers(headers)
//             .multipart(form)
//             .send()
//             .await?;
//
//         info!("Update file to server response.status: {}", resp.status());
//     }
//     Ok(())
// }
// }

// #[derive(Debug, Serialize, Deserialize, Clone)]
// pub struct LogCheckResp {
//     #[serde(rename = "startTime")]
//     start_time: Option<i64>,
//     #[serde(rename = "endTime")]
//     end_time: Option<i64>,
//     #[serde(rename = "requestId")]
//     request_id: Option<String>,
//     #[serde(rename = "needUpload")]
//     need_upload: bool,
// }

// impl Default for LogCheckResp {
//     fn default() -> Self {
//         Self {
//             start_time: None,
//             end_time: None,
//             request_id: None,
//             need_upload: false,
//         }
//     }
// }

// impl LogCheckResp {
//     async fn search_upload_file(&self, path_buf: &PathBuf, app_info: &AppInfo) {
//         if self.start_time.is_none() && self.end_time.is_none() {
//             return;
//         }
//
//         let if_present = |file_time: DateTime<Utc>| {
//             let timestamp = file_time.timestamp_millis();
//             timestamp >= self.start_time.unwrap() && self.end_time.unwrap() >= timestamp
//         };
//
//         let file_entries = file_util::load_file_entries(path_buf);
//         if file_entries.is_empty() {
//             warn!("The log file is empty, there is no need to upload it to the server.");
//             return;
//         }
//
//         for file_entry in file_entries.iter() {
//             let file_time = file_util::get_log_file_update_time(file_entry);
//             debug!("File[{:?}] update time[{}]", &file_entry.path(), &file_time);
//
//             let filepath = file_entry.path();
//             let path_str = filepath.to_str().unwrap();
//             if !(if_present(file_time) && path_str.contains("default.log")) {
//                 warn!("File[{:?}] that do not meet the query time.", &file_entry.path());
//                 continue;
//             }
//
//             let log_msg = LogMsg::new(&app_info, self.request_id.to_owned());
//             if let Err(err) = log_msg.send_file_and_data(&filepath).await {
//                 error!("Update file[{:?}] to server error: {:?}", &filepath, err);
//             }
//         }
//     }
// }

// #[derive(Debug, Clone)]
// pub struct LogFile(PathBuf);

// impl LogFile {
//     pub fn new(path_buf: PathBuf) -> Self {
//         Self(path_buf)
//     }
//
//     pub async fn check_need_upload(&self) {
//         let app_ops = match oauths::query_app_info().await {
//             Ok(app_ops) => app_ops,
//             Err(err) => {
//                 warn!("Upload log file to server query AppInfo error: {:?}", err);
//                 None
//             }
//         };
//
//         let upload_if_present = |resp: LogCheckResp| async move {
//             if !resp.need_upload {
//                 return;
//             }
//             if let Some(app_info) = app_ops {
//                 resp.search_upload_file(&self.0, &app_info).await;
//             }
//         };
//
//         match check_log_request().await {
//             Ok(resp) => {
//                 upload_if_present(resp).await;
//             }
//             Err(err) => {
//                 error!("check_log_request error: {:?}", err);
//             }
//         }
//     }
// }

// pub async fn check_log_request() -> Result<LogCheckResp, CommonError> {
//     let ser_url = SettingsCode::ServerUrl.get_value().await;
//     let check_uri = SettingsCode::LogCheckUri.get_value().await;
//     let url = format!("{}{}", ser_url, check_uri);
//
//     let headers = build_botser_headers().await?;
//     let client = ReqwestClient::build();
//     let response = client.get(url).headers(headers).send().await?;
//
//     let base_resp = response.json::<BaseResp<LogCheckResp>>().await?;
//     debug!("check server log request base_resp: {:?}", base_resp);
//     match base_resp.data {
//         Some(value) => {
//             info!("check server log request info: {:?}", &value);
//             Ok(value)
//         }
//         None => {
//             warn!("check server log request, LogCheckResp is none, return default.");
//             warn!("server error: {}:{:?}", base_resp.code, base_resp.msg);
//             Ok(LogCheckResp::default())
//         }
//     }
// }


// async fn build_botser_headers() -> Result<HeaderMap, CommonError> {
//     let api_key = LocalApiKey::get().await?;
//     let mut headers = HeaderMap::new();
//     headers.insert(
//         HeaderName::from_static("serverapikey"),
//         HeaderValue::from_str(api_key.ser_api_key.as_str())?,
//     );
//     Ok(headers)
// }
