// #[cfg(not(target_env = "msvc"))]
// use jemallocator::Jemalloc;

use log::info;
use ntex::server::Server;
use ntex::web::{App, HttpServer};
use ntex::web;
use ntex::web::{ServiceConfig};
use ntex_auth::auth::auth_api;
use ntex_auth::middleware::auth_filter;
use crate::trace::trace_api;

// #[cfg(not(target_env = "msvc"))]
// #[global_allocator]
// static GLOBAL: Jemalloc = Jemalloc;

// #[global_allocator]
// static GLOBAL: jemallocator::Jemalloc = jemallocator::Jemalloc;

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
    }).workers(4).bind(&bind)?.run().await
}

/// api入口
pub fn ser_config(cfg: &mut ServiceConfig) {
    cfg.service(web::scope("/api").service((auth_api::login, trace_api::store_log)));
}
