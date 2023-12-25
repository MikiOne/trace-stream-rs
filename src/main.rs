use std::env;

use log::info;
use ntex::web::{App, HttpServer, middleware};

use oasis_log_collector::log_monitor;
use oasis_log_collector::settings::Settings;

#[ntex::main]
async fn main() -> std::io::Result<()> {
    let config = Settings::new().expect("读取配置文件出错");

    env::set_var("RUST_BACKTRACE", "1");
    if config.is_debug() {
        env::set_var("RUST_LOG", "debug");
    } else {
        env::set_var("RUST_LOG", "info");
    }
    env_logger::init();
    log_monitor::init_monitor(config);

    let bind = "127.0.0.1:18283";
    info!("Starting server at: {}", &bind);

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
        // .configure(user_handler::config)
        // .service((user::login, user::logout))
    })
        .bind(&bind)?
        .run()
        .await
}

