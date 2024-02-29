// use std::collections::HashMap;
// use std::io::{self, BufWriter, Write};
// use std::path::PathBuf;
// use std::sync::Arc;
//
// use flate2::Compression;
// use flate2::write::GzEncoder;
// use log::{debug, error, warn};
// use regex::Regex;
//
// use tokio::sync::OnceCell;
// use tokio::fs::OpenOptions;
// use tokio::io::AsyncWriteExt;
// use tokio::sync::{mpsc, Mutex};
// use tokio::sync::mpsc::Receiver;
// use tokio::fs::File;
// use once_cell::sync::Lazy;
//
// use common::data_utils::to_previous_day;
// use common::file_utils::create_dir;
// use common::models::LogBody;
//
//
// pub async fn store(log: &LogBody) {
//     let store_path = STORE_PATH.get().unwrap();
//     debug!("日志存储路径: {}", store_path.display());
//     let store_path = store_path.join(format!("{}/", log.project_name));
//
//     debug!("日志存储目录: {}", store_path.display() );
//     // 创建目录
//     create_dir(store_path.as_path());
//
//     let map_key = format!("{}-{}", log.project_name, log.server_name);
//     let filename = format!("{}-{}.log", log.server_name, log.log_day);
//     debug!("日志文件: {}", &filename);
//     let logfile = store_path.join(filename);
//     let mut map = GLOBAL_MAP.lock().await;
//
//     let header = format!("[{}:{}]", log.server_ip, log.server_name);
//     let lines = log.log_info.split("\n");
//     let log_lines: Vec<String> = lines
//         .map(|line| if starts_with_date(line)
//         { format!("{} {}", header, line) } else { line.to_string() })
//         .collect();
//     let (tx, rx) = mpsc::channel::<Vec<String>>(100);
//     tx.clone().send(log_lines).await.unwrap();
//
//     if logfile.exists() {
//         let file_writer = Arc::new(Mutex::new(map.get(&map_key).unwrap()));
//         let file = file_writer.clone();
//         let file = file.lock().await;
//         let file = file.clone();
//         write_async(file, rx).await;
//     } else {
//         // 打开文件一次，以追加模式写入
//         let file = OpenOptions::new().create(true).write(true).append(true).open(logfile).await.unwrap();
//         map.insert(map_key.clone(), Arc::new(Mutex::new(file)));
//
//         let file_writer = Arc::new(Mutex::new(map.get(&map_key).unwrap()));
//         let file = file_writer.clone();
//         let file = file.lock().await;
//         let file = file.clone();
//         write_async(file, rx).await;
//         // 如果创建新的文件，则需要压缩旧的文件
//         compress_old_file(log);
//
//         // remove old from map
//         {
//             let mut map = GLOBAL_MAP.lock().await;
//             map.remove(&map_key);
//         }
//     }
// }
//
//
//
//
//
// // #[test]
// // fn test_compress_file() {
// //     let input_path = "./logs/default.log-20240222-0.log";  // 输入文件
// //     let input_path = PathBuf::from(input_path);
// //     // 对文件进行压缩
// //     compress_file(&input_path);
// // }