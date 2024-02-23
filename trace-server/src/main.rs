use std::path::PathBuf;

use log::{error, info};
use ntex::web::{App, HttpServer};
use ntex::web;
use ntex::web::{HttpResponse, Responder, ServiceConfig};
use ntex::web::types::Json;
use regex::Regex;

use common::biz_resp::RespData;
use common::models::LogBody;

use crate::log4rs_config::ConfigLog4rs;
use crate::settings::Settings;
use crate::store_compress::{init_store_path, store};

pub mod log4rs_config;
mod store_compress;
mod settings;

#[ntex::main]
async fn main() -> std::io::Result<()> {
    let settings = Settings::new().expect("读取配置文件出错");
    let store_path = settings.store_path();
    let log_path = PathBuf::from(store_path.clone());

    init_store_path(&log_path).await;
    ConfigLog4rs::new(&log_path).unwrap().init_config().unwrap();

    let bind = "0.0.0.0:7200";
    info!("Starting server at: {}", &bind);

    HttpServer::new(move || {
        App::new().configure(ser_config)
    }).bind(&bind)?.run().await
}

/// api入口
pub fn ser_config(cfg: &mut ServiceConfig) {
    cfg.service(web::scope("/api/trace").service((collect, )));
}

#[web::post("/collect")]
pub async fn collect(
    log: Json<LogBody>,
) -> Result<impl Responder, web::Error> {
    store(&log);
    info!("Logs saved successfully: {}", log.print_base());
    Ok(HttpResponse::Ok().json(&RespData::success()))
}

// #[web::post("/collect")]
// pub async fn collect(
//     log: Json<LogBody>,
// ) -> Result<impl Responder, web::Error> {
//     let header = format!("[{}:{}]", log.server_ip, log.server_name);
//     let mut lines = log.log_info.split("\n");
//     lines.by_ref().for_each(|line| {
//         if starts_with_date(line) {
//             error!("{} {}", header, line);
//         } else {
//             error!("{}", line);
//         }
//         // error!("{} {}", header, line);
//     });
//     Ok(HttpResponse::Ok().json(&RespData::success()))
// }
