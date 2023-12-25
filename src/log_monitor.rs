use std::rc::Rc;
use std::sync::Arc;

use anyhow::Result;
use log::{error, info};

use crate::settings::{Eoplog, Settings};

pub fn init_monitor(config: Settings) {
    for eoplog in config.get_eoplogs() {
        // let path_buf = PathBuf::from(eoplog.get_path());
        LogFileMonitor::new(&eoplog).unwrap().init_watcher();
        info!("LogFileMonitor init success: {:?}", &eoplog)
    }
}

pub struct LogFileMonitor(Rc<Eoplog>);

impl LogFileMonitor {
    pub fn new(eoplog: &Eoplog) -> Result<LogFileMonitor> {
        Ok(LogFileMonitor(Rc::new(eoplog.to_owned())))
    }

    // log watcher init
    pub fn init_watcher(&self) {
        let rc_log = Rc::clone(&self.0);
        let default_log = rc_log.get_path();
        let mut log_watcher = async_log_watcher::LogWatcher::new(default_log);
        let log_watcher_handle = log_watcher.spawn(true);

        tokio::task::spawn(async {
            if let Err(err) = log_watcher_handle.await {
                error!("log_watcher_handle await error: {:?}", err);
            }
        });

        let ser_name = Arc::new(rc_log.get_ser_name().to_owned());
        tokio::task::spawn(async move {
            while let Some(data) = log_watcher.read_message().await {
                println!("3333333");
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
    info!("Collection logs: file name[{:?}] \n{}", ser_name, msg)
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

