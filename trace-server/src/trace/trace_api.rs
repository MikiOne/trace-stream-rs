use log::info;
use ntex::web;
use ntex::web::{HttpResponse, Responder};
use ntex::web::types::Json;

use common::biz_resp::RespData;
use common::models::LogBody;
use crate::trace::async_store_compress;

#[web::post("/trace/collect")]
pub async fn store_log(
    log: Json<LogBody>,
) -> Result<impl Responder, web::Error> {
    async_store_compress::store(&log).await;
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
