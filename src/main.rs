use std::env;


use oasis_log_collector::log_monitor;
use oasis_log_collector::settings::Settings;

#[tokio::main]
async fn main() {
    let config = Settings::new().expect("读取配置文件出错");

    env::set_var("RUST_BACKTRACE", "1");
    if config.is_debug() {
        env::set_var("RUST_LOG", "debug");
    } else {
        env::set_var("RUST_LOG", "info");
    }
    env_logger::init();
    log_monitor::init_monitor(config);

    // let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();

    loop {
        // match listener.accept().await {
        //     Ok((socket, addr)) => {
        //         tokio::spawn(async move {
        //             // 此处处理连接
        //         });
        //     }
        //     Err(e) => {
        //         // 打印错误并继续监听
        //         println!("Failed to accept connection: {}", e);
        //     }
        // }
    }
}

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
//     let app = Router::new().route("/", get(handler));
//     let app = app.fallback(handler_404);
//
//     let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
//     println!("listening on {}", listener.local_addr().unwrap());
//     axum::serve(listener, app).await.unwrap();
// }
// async fn handler() -> Html<&'static str> {
//     Html("<h1>Nothing!</h1>")
// }
//
// async fn handler_404() -> impl IntoResponse {
//     (StatusCode::NOT_FOUND, "nothing to see here")
// }

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
//     app.listen("127.0.0.1:8080").await?;
//     Ok(())
// }
