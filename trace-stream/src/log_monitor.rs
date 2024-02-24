use std::rc::Rc;
use std::sync::Arc;
use tokio::sync::OnceCell;

use anyhow::Result;
use common::log::{error, info};
use common::models::LogBody;
use log_upload::api_upload::{send_log};
use log_upload::http_client;
use log_upload::settings::{LogInfo, RemoteServerConfig};

pub static REMOTE_SERVER: OnceCell<RemoteServerConfig> = OnceCell::const_new();

pub async fn init_monitor(log_infos: Vec<LogInfo>, remote_server_config: RemoteServerConfig) {
    REMOTE_SERVER.get_or_init(|| async { remote_server_config.to_owned() }).await;

    for log_info in log_infos {
        // let path_buf = PathBuf::from(eoplog.get_path());
        LogInfoMonitor::new(&log_info).unwrap().init_watcher();
        info!("LogFileMonitor init success: {:?}", &log_info)
    }
}

pub struct LogInfoMonitor {
    log_info: Rc<LogInfo>,
}

impl LogInfoMonitor {
    pub fn new(log_info: &LogInfo) -> Result<LogInfoMonitor> {
        Ok(LogInfoMonitor { log_info: Rc::new(log_info.to_owned()) })
    }

    // log watcher init
    pub fn init_watcher(&self) {
        let rc_log = Rc::clone(&self.log_info);
        let default_log = rc_log.get_path();
        let mut log_watcher = async_log_watcher::LogWatcher::new(default_log);
        let log_watcher_handle = log_watcher.spawn(true);

        tokio::task::spawn(async {
            if let Err(err) = log_watcher_handle.await {
                error!("log_watcher_handle await error: {:?}", err);
            }
        });

        let ser_name = Arc::new(rc_log.get_server_name().to_owned());
        tokio::task::spawn(async move {
            while let Some(data) = log_watcher.read_message().await {
                match std::str::from_utf8(&data) {
                    Ok(data) => send_msg(data, Arc::clone(&ser_name)).await,
                    Err(err) => error!("read_message parse data error: {:?}", err),
                }
            }
        });

        // init check_server_log_request
        // self.init_need_upload_logfile_job();
    }

    // fn init_need_upload_logfile_job(&self) {
    //     info!("init init_need_upload_logfile_job success");
    //     let log_file = LogFile::new(self.0.clone());
    //     tokio::task::spawn(async move {
    //         let mut interval = time::interval(time::Duration::from_secs(10 * 60));
    //         // let mut interval = time::interval(time::Duration::from_secs(1 * 60));
    //         loop {
    //             interval.tick().await;
    //             info!("Scheduled checks if files need to be uploaded.");
    //
    //             log_file.check_need_upload().await;
    //         }
    //     });
    // }
}

async fn send_msg(msg: &str, ser_name: Arc<String>) {
    info!("Collection logs: file name[{:?}] \n{}", ser_name, msg);
    let pub_ip = http_client::get_pub_ip_str().await;
    let log_body = LogBody::new(ser_name.to_string(), pub_ip.to_string(), msg.to_string());

    match REMOTE_SERVER.get() {
        Some(remote_server) =>
            send_log(remote_server, &log_body).await.expect("Watcher send msg to server error: "),
        None =>
            error!("Watcher send msg to server error: RemoteServerConfig not found."),
    }
}

// async fn send_msg(msg: &str) {
//     match oauths::query_app_info().await {
//         Ok(app_op) => {
//             if let Some(app) = app_op {
//                 let msg = Some(format!("\n{}", msg.to_string()));
//                 if let Err(err) = LogMsg::new(&app, msg).send_msg().await {
//                     warn!("Send msg to server error: {:?}", err);
//                 }
//             } else {
//                 warn!("Send msg to server error: AppInfo not found.");
//             }
//         }
//         Err(err) => warn!("Send msg to server query AppInfo error: {:?}", err),
//     }
// }

