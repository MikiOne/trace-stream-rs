use std::env;
use axum::Router;
use axum::routing;
use tokio::net::TcpListener;
// use msg_pub::kafka_client::KafkaPub;

use trace_stream_rs::log_monitor;
use common::settings::Settings;

#[tokio::main]
async fn main() {
    let config = Settings::new().expect("读取配置文件出错");

    env::set_var("RUST_BACKTRACE", "    1");
    if config.is_debug() {
        env::set_var("RUST_LOG", "debug");
    } else {
        env::set_var("RUST_LOG", "info");
    }
    common::env_logger::init();
    log_monitor::init_monitor(config.log_infos, config.remote_server).await;

    // let kafka_producer = KafkaPub::new(config.kafka_config);
    // test
    // kafka_producer.produce("my-topic", "ni好友").await;

    let app = Router::new().route("/", routing::get(|| async {}));
    let listener = TcpListener::bind("127.0.0.1:13000").await.expect("tcp port bind error");
    axum::serve(listener, app).await.expect("Axum server start error");
}

// ntex web
// #[ntex::main]
// async fn main() -> std::io::Result<()> {
//     let config = Settings::new().expect("读取配置文件出错");
//
//     env::set_var("RUST_BACKTRACE", "1");
//     if config.is_debug() {
//         env::set_var("RUST_LOG", "debug");
//     } else {
//         env::set_var("RUST_LOG", "info");
//     }
//     env_logger::init();
//     log_monitor::init_monitor(config);
//
//     let bind = "127.0.0.1:8080";
//     HttpServer::new(move || {
//         App::new()
//     }).bind(&bind)?.run().await
// }

// error: there is no reactor running, must be called from the context of a Tokio 1.x runtime
// tide web
// #[async_std::main]
// async fn main() -> tide::Result<()> {
//     let config = Settings::new().expect("读取配置文件出错");
//
//     env::set_var("RUST_BACKTRACE", "1");
//     if config.is_debug() {
//         env::set_var("RUST_LOG", "debug");
//     } else {
//         env::set_var("RUST_LOG", "info");
//     }
//     env_logger::init();
//     log_monitor::init_monitor(config);
//
//     let mut app = tide::new();
//     app.listen("127.0.0.1:18182").await?;
//     Ok(())
// }


// use warp::Filter;
// #[tokio::main]
// async fn main() {
//     let config = Settings::new().expect("读取配置文件出错");
//
//     env::set_var("RUST_BACKTRACE", "1");
//     if config.is_debug() {
//         env::set_var("RUST_LOG", "debug");
//     } else {
//         env::set_var("RUST_LOG", "info");
//     }
//     env_logger::init();
//     log_monitor::init_monitor(config);
//
//     let hello = warp::path!("hello" / String)
//         .map(|name| format!("Hello, {}!", name));
//
//     warp::serve(hello).run(([127, 0, 0, 1], 13030)).await;
// }


// tokio tcp
// use tokio::net::TcpListener;
// #[tokio::main]
// async fn main()  -> io::Result<()> {
//     let config = Settings::new().expect("读取配置文件出错");
//
//     env::set_var("RUST_BACKTRACE", "1");
//     if config.is_debug() {
//         env::set_var("RUST_LOG", "debug");
//     } else {
//         env::set_var("RUST_LOG", "info");
//     }
//     env_logger::init();
//     log_monitor::init_monitor(config);
//
//     let listener = TcpListener::bind("127.0.0.1:8080").await?;
//     loop {
//         let (_, _) = listener.accept().await?;
//     }
// }