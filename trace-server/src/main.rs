// #[cfg(not(target_env = "msvc"))]
// use jemallocator::Jemalloc;

use log::info;
use ntex::web::{App, HttpServer};
use ntex::web;
use ntex::web::ServiceConfig;
use tokio::sync::mpsc;

use ntex_auth::auth::auth_api;
use ntex_auth::middleware::auth_filter;
use crate::trace::file_store::{FileData, FileDataSender};

use crate::trace::{file_store, trace_api};

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

    let (tx, mut rx) = mpsc::channel::<FileData>(10);
    // 将 sender 设为全局
    FileDataSender::init(tx).await;
    tokio::spawn(async move {
        while let Some(file_data) = rx.recv().await {
            file_store::store_data(file_data).await;
        }
    });

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
