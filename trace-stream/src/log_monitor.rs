use std::rc::Rc;
use std::sync::Arc;
use tokio::sync::OnceCell;

use anyhow::Result;
use common::log::{debug, error, info};
use common::models::{LogBody, LogInfo};
use crate::publish::api_upload::{send_log};
use crate::publish::http_client;
use crate::setup::REMOTE_SERVER;


pub async fn init_monitor(log_infos: Vec<LogInfo>) {
    for log_info in log_infos {
        LogInfoMonitor::new(&log_info).unwrap().init_watcher();
        info!("LogFileMonitor init success: {:?}", &log_info)
    }
}

pub struct LogInfoMonitor {
    log_info: Arc<LogInfo>,
}

impl LogInfoMonitor {
    pub fn new(log_info: &LogInfo) -> Result<LogInfoMonitor> {
        Ok(LogInfoMonitor { log_info: Arc::new(log_info.to_owned()) })
    }

    // log watcher init
    pub fn init_watcher(&self) {
        let rc_log = Arc::clone(&self.log_info);
        let default_log = rc_log.get_path();
        let mut log_watcher = async_log_watcher::LogWatcher::new(default_log);
        let log_watcher_handle = log_watcher.spawn(true);

        tokio::task::spawn(async {
            if let Err(err) = log_watcher_handle.await {
                error!("log_watcher_handle await error: {:?}", err);
            }
        });

        tokio::task::spawn(async move {
            while let Some(data) = log_watcher.read_message().await {
                match std::str::from_utf8(&data) {
                    Ok(data) => send_msg(data, Arc::clone(&rc_log)).await,
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

async fn send_msg(msg: &str, log_info: Arc<LogInfo>) {
    debug!("Collection logs: file log_info[{:?}] \n{}", &log_info, msg);
    let pub_ip = http_client::get_pub_ip_str().await;
    let log_body = LogBody::new(Arc::clone(&log_info), pub_ip.to_string(), msg.to_string());

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

