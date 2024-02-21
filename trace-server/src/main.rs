#[macro_use]
extern crate log;
extern crate log4rs;

use std::path::PathBuf;

use log::info;
use ntex::web::{App, HttpServer, middleware};
use ntex::web;
use ntex::web::{HttpResponse, Responder, ServiceConfig};
use ntex::web::types::Json;
use rand::Rng;

use common::biz_resp::RespData;
use common::models::LogBody;

use crate::log4rs_config::ConfigLog4rs;

pub mod log4rs_config;

#[ntex::main]
async fn main() -> std::io::Result<()> {
    let log_path = PathBuf::from("./logs");
    ConfigLog4rs::new(&log_path).unwrap().init_config().unwrap();

    let bind = "127.0.0.1:7200";
    info!("Starting server at: {}", &bind);

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
    })
        .bind(&bind)?
        .run()
        .await
}

/// api入口
pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(web::scope("/api/trace").service((collect, )));
}

#[web::post("/collect")]
pub async fn collect(
    log_info: Json<LogBody>,
) -> Result<impl Responder, web::Error> {
    info!("Json<LogBody>: {log_info}");
    Ok(HttpResponse::Ok().json(&RespData::success()))
}