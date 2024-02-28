use ntex::web;
use ntex::web::{App, HttpServer, ServiceConfig};

use common::log::info;
use ntex_auth::auth::auth_api;
use ntex_auth::middleware::auth_filter;

// #[global_allocator]
// static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

mod log_monitor;
mod publish;
pub mod setup;
pub mod settings;

#[ntex::main]
async fn main() -> std::io::Result<()> {
    setup::init().await;

    let bind = "0.0.0.0:7201";
    info!("Starting server at: {}", &bind);

    HttpServer::new(move || {
        App::new().wrap(auth_filter::JwtFilter).configure(ser_config)
    }).bind(&bind)?.run().await
}

/// api入口
pub fn ser_config(cfg: &mut ServiceConfig) {
    cfg.service(web::scope("/api").service((auth_api::login,)));
}
