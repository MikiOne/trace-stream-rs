use std::collections::HashMap;
use std::io::{self, BufWriter, Write};
use std::path::PathBuf;
use std::sync::Arc;

use flate2::Compression;
use flate2::write::GzEncoder;
use log::{debug, error, warn};
use regex::Regex;

use tokio::sync::OnceCell;
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;
use tokio::sync::{mpsc, Mutex};
use tokio::sync::mpsc::Receiver;
use tokio::fs::File;
use once_cell::sync::Lazy;

use common::data_utils::to_previous_day;
use common::file_utils::create_dir;
use common::models::LogBody;

static GLOBAL_MAP: Lazy<Mutex<HashMap<String, File>>> = Lazy::new(|| {
    let m = HashMap::new();
    Mutex::new(m)
});

pub static STORE_PATH: OnceCell<PathBuf> = OnceCell::const_new();

// let (tx, rx) = mpsc::channel::<Vec<String>>(100);
pub async fn init_store_path(path: &PathBuf) {
    STORE_PATH.get_or_init(|| async { path.to_owned() }).await;
}

fn starts_with_date(s: &str) -> bool {
    let date_regex = Regex::new(r"^\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}").unwrap();
    let date_s_regex = Regex::new(r"^\[\d{4}-\d{2}-\d{2}").unwrap();
    let date_t_regex = Regex::new(r"^\[\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}").unwrap();
    date_regex.is_match(s) || date_s_regex.is_match(s) || date_t_regex.is_match(s)
}

pub async fn store(log: &LogBody) {
    let store_path = STORE_PATH.get().unwrap();
    debug!("日志存储路径: {}", store_path.display());
    let store_path = store_path.join(format!("{}/", log.project_name));

    debug!("日志存储目录: {}", store_path.display() );
    // 创建目录
    create_dir(store_path.as_path());

    let map_key = format!("{}-{}", log.project_name, log.server_name);
    let filename = format!("{}-{}.log", log.server_name, log.log_day);
    debug!("日志文件: {}", &filename);
    let logfile = store_path.join(filename);
    let mut map = GLOBAL_MAP.lock().await;


    let header = format!("[{}:{}]", log.server_ip, log.server_name);
    let lines = log.log_info.split("\n");
    let log_lines: Vec<String> = lines
        .map(|line| if starts_with_date(line)
        { format!("{} {}", header, line) } else { line.to_string() })
        .collect();
    let (tx, rx) = mpsc::channel::<Vec<String>>(100);
    tx.clone().send(log_lines).await.unwrap();

    if logfile.exists() {
        let file_writer = Arc::new(Mutex::new(map.get(&map_key).unwrap()));
        write_async(file_writer.clone(), rx).await;
    } else {
        // 打开文件一次，以追加模式写入
        let file = OpenOptions::new().create(true).write(true).append(true).open(logfile).await.unwrap();
        map.insert(map_key.clone(), file);

        let file_writer = Arc::new(Mutex::new(map.get(&map_key).unwrap()));
        write_async(file_writer.clone(), rx).await;
        // 如果创建新的文件，则需要压缩旧的文件
        compress_old_file(log);

        // remove old from map
        map.remove(&map_key);
    }
}


pub async fn write_async(file_writer: Arc<Mutex<File>>, mut rx: Receiver<Vec<String>>) {
    let writer_clone = file_writer.clone();
    tokio::spawn(async move {
        while let Some(log_lines) = rx.recv().await {
            let mut file = writer_clone.lock().await;
            for line in log_lines {
                if line.len() > 0 {
                    file.write_all(line.as_bytes()).await.unwrap();
                }
            }
            file.flush().await.unwrap();
        }
    }).await.unwrap();
}


/// 已追加的方式写入数据
// fn write_with_append(logfile: &PathBuf, log_lines: Vec<String>) {
//     match OpenOptions::new().append(true).open(logfile) {
//         Ok(file) => {
//             writeln(file, log_lines);
//         }
//         Err(err) => {
//             error!("打开文件失败: {}", err);
//             warn!("{}\n{}", logfile.display(), log_lines.join("\n"));
//         }
//     }
// }

// fn write_with_create(logfile: &PathBuf, log_lines: Vec<String>) {
//     match File::create(logfile) {
//         Ok(file) => {
//             writeln(file, log_lines)
//         }
//         Err(err) => {
//             error!("创建文件失败: {}", err);
//             warn!("{}\n{}", logfile.display(), log_lines.join("\n"));
//         }
//     }
// }

// fn writeln(file: File, log_lines: Vec<String>) {
//     let mut writer = BufWriter::new(file);
//     for line in log_lines {
//         if line.len() > 0 {
//             writeln!(writer, "{}", line).unwrap();
//         }
//     }
//     writer.flush().unwrap();
// }

fn compress_old_file(log: &LogBody) {
    let store_path = STORE_PATH.get().unwrap()
        .join(format!("{}/", log.project_name));
    let pre_day = to_previous_day(log.log_day.as_str());
    match pre_day {
        Some(pre_day) => {
            let old_logfile = store_path.join(format!("{}-{}.log", log.server_name, pre_day));
            compress_file(&old_logfile);
        }
        None => {
            warn!("无法获取前一天的日期: {}", log.log_day)
        }
    }
}

fn compress_file(logfile: &PathBuf) {
    if !logfile.exists() {
        warn!("压缩前一天的日志文件不存在: {}", logfile.display());
        return;
    }

    let output_path = format!("{}.gz", logfile.display());
    // 打开待压缩的文件
    let mut input_file = std::fs::File::open(logfile).expect("打开待压缩的文件");

    // 创建一个写文件操作，以写入压缩的数据
    let output_file = std::fs::File::create(output_path).expect("创建压缩文件失败");

    // 创建Gz编码器，用于压缩数据。
    let mut encoder = GzEncoder::new(output_file, Compression::default());

    // 将内容复制到编码器中，它会将压缩后的数据写入到输出文件
    io::copy(&mut input_file, &mut encoder).expect("压缩失败");

    // 完成压缩并刷新任何剩余的输出
    encoder.finish().expect("压缩失败");

    // 删除原文件
    std::fs::remove_file(logfile).expect("删除文件失败");
}


// #[test]
// fn test_compress_file() {
//     let input_path = "./logs/default.log-20240222-0.log";  // 输入文件
//     let input_path = PathBuf::from(input_path);
//     // 对文件进行压缩
//     compress_file(&input_path);
// }