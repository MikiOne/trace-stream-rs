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

pub mod log4rs_config;

#[ntex::main]
async fn main() -> std::io::Result<()> {
    let log_path = PathBuf::from("./logs");
    ConfigLog4rs::new(&log_path).unwrap().init_config().unwrap();

    let bind = "0.0.0.0:7200";
    info!("Starting server at: {}", &bind);

    HttpServer::new(move || {
        App::new()
            .configure(config)
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
    log: Json<LogBody>,
) -> Result<impl Responder, web::Error> {
    let header = format!("[{}:{}]", log.server_ip, log.server_name);
    let mut lines = log.log_info.split("\n");
    lines.by_ref().for_each(|line| {
        if starts_with_date(line) {
            error!("{} {}", header, line);
        } else {
            error!("{}", line);
        }
        // error!("{} {}", header, line);
    });
    Ok(HttpResponse::Ok().json(&RespData::success()))
}

fn starts_with_date(s: &str) -> bool {
    let date_regex = Regex::new(r"^\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}").unwrap();
    date_regex.is_match(s)
}