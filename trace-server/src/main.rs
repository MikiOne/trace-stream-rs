use std::path::PathBuf;

use log::info;
use ntex::web::{App, HttpServer};
use ntex::web;
use ntex::web::{Responder, ServiceConfig};

use common::log4rs_config::ConfigLog4rs;
use ntex_auth::auth::auth_api;
use ntex_auth::middleware::auth_filter;

use crate::settings::Settings;
use crate::trace::store_compress::init_store_path;
use crate::trace::trace_api;

mod settings;
mod trace;
mod setup;

#[ntex::main]
async fn main() -> std::io::Result<()> {
    setup::init().await;

    let bind = "0.0.0.0:7200";
    info!("Starting server at: {}", &bind);

    HttpServer::new(move || {
        App::new().wrap(auth_filter::JwtFilter).configure(ser_config)
    }).bind(&bind)?.run().await
}

/// api入口
pub fn ser_config(cfg: &mut ServiceConfig) {
    cfg.service(web::scope("/api").service((auth_api::login, trace_api::store_log)));
}
