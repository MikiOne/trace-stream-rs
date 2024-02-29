use std::collections::HashMap;
use std::io;
use std::path::PathBuf;
use std::sync::Arc;
use chrono::NaiveDate;
use flate2::Compression;
use flate2::write::GzEncoder;

use log::{debug, info, warn};
use once_cell::sync::Lazy;
use regex::Regex;
use tokio::fs::{File, OpenOptions};
use tokio::io::{AsyncWriteExt, BufWriter};
use tokio::sync::{Mutex, OnceCell};
use tokio::sync::mpsc::Sender;
use common::data_utils::to_previous_day;

use common::file_utils::create_dir;
use common::models::LogBody;

static GLOBAL_SENDER: OnceCell<FileDataSender> = OnceCell::const_new();

static GLOBAL_MAP: Lazy<Mutex<HashMap<String, Arc<Mutex<File>>>>> = Lazy::new(|| {
    let m = HashMap::new();
    Mutex::new(m)
});

pub static STORE_PATH: OnceCell<PathBuf> = OnceCell::const_new();

pub async fn init_store_path(path: &PathBuf) {
    STORE_PATH.get_or_init(|| async { path.to_owned() }).await;
}

pub struct FileData {
    pub file: Arc<Mutex<File>>,
    pub logs: Vec<String>,
}

impl FileData {
    pub fn new(file: Arc<Mutex<File>>, logs: Vec<String>) -> Self {
        Self { file, logs }
    }
}


pub struct FileDataSender {
    output_tx: Box<Sender<FileData>>,
}

impl FileDataSender {
    pub async fn init(output_tx: Sender<FileData>) {
        GLOBAL_SENDER.get_or_init(|| async {
            Self { output_tx: Box::new(output_tx) }
        }).await;
    }

    fn get(&self) -> Box<Sender<FileData>> {
        self.output_tx.clone()
    }

    pub fn get_sender() -> Result<Box<Sender<FileData>>, String> {
        let sender = GLOBAL_SENDER.get();
        if let Some(sender) = sender {
            Ok(sender.get())
        } else {
            Err("FileDataSender not init".to_string())
        }
    }
}


pub async fn send_file_data(log: &LogBody) {
    let store_path = STORE_PATH.get().unwrap();
    debug!("日志存储路径: {}", store_path.display());
    let store_path = store_path.join(format!("{}/", log.project_name));

    debug!("日志存储目录: {}", store_path.display() );
    // 创建目录
    create_dir(store_path.as_path());

    let filename = format!("{}-{}.log", log.server_name, log.log_day);
    let logfile = store_path.join(filename);
    info!("日志文件: {}", &logfile.display());

    let header = format!("[{}:{}]", log.server_ip, log.server_name);
    let lines = log.log_info.split("\n");
    let log_lines: Vec<String> = lines
        .map(|line| if starts_with_date(line)
        { format!("{} {}", header, line) } else { line.to_string() })
        .collect();

    let map_key = format!("{}-{}-{}", log.project_name, log.server_name, log.log_day);
    let arc_file = get_opened_file(&map_key, &logfile, log).await;
    let file_data = FileData::new(arc_file, log_lines);

    let sender = FileDataSender::get_sender().unwrap();
    sender.send(file_data).await.expect("Send log data to file error: ");

    // 处理历史文件
    handle_old_file(log, &logfile).await;
}


async fn get_opened_file(map_key: &String, logfile: &PathBuf, log: &LogBody) -> Arc<Mutex<File>> {
    let mut map = GLOBAL_MAP.lock().await;
    match map.get(map_key) {
        None => {
            let file = OpenOptions::new().create(true).write(true).append(true).open(logfile).await.unwrap();
            let file = Arc::new(Mutex::new(file));
            map.insert(map_key.to_owned(), file.clone());
            file
        }
        Some(file) => {
            file.clone()
        }
    }
}


pub async fn store_data(file_data: FileData) {
    let file_writer = file_data.file;
    let mut file = file_writer.lock().await;
    let log_lines = file_data.logs;

    for line in log_lines {
        if line.len() > 0 {
            let line_ln = format!("{}\n", line);
            file.write_all(line_ln.as_bytes()).await.unwrap();
        }
    }
    file.flush().await.unwrap();
}


fn starts_with_date(s: &str) -> bool {
    let date_regex = Regex::new(r"^\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}").unwrap();
    let date_s_regex = Regex::new(r"^\[\d{4}-\d{2}-\d{2}").unwrap();
    let date_t_regex = Regex::new(r"^\[\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}").unwrap();
    date_regex.is_match(s) || date_s_regex.is_match(s) || date_t_regex.is_match(s)
}


async fn handle_old_file(log: &LogBody, logfile: &PathBuf) {
    if !is_le_00_30() {
        return;
    }

    let pre_logfile = build_preday_file(logfile, log);
    if let Some((pre_day, old_logfile)) = pre_logfile {
        if old_logfile.exists() {
            // 需要压缩旧的文件
            compress_file(&old_logfile);

            let map_key = format!("{}-{}-{}", log.project_name, log.server_name, pre_day);
            remove_preday_file(map_key).await;
        }
    } else {
        warn!("无法获取[{}]前一天的日期", log.log_day)
    }
}

async fn remove_preday_file(map_key: String) {
    let mut map = GLOBAL_MAP.lock().await;
    map.remove(&map_key);
}

fn build_preday_file(logfile: &PathBuf, log: &LogBody) -> Option<(NaiveDate, PathBuf)> {
    let log_parent = logfile.parent().unwrap();
    let pre_day = to_previous_day(log.log_day.as_str());

    if let Some(pre_day) = pre_day {
        let old_file = log_parent.join(format!("{}-{}.log", log.server_name, pre_day));
        info!("build preday file old_file: {}", old_file.display());
        Some((pre_day, old_file))
    } else {
        None
    }
}

fn compress_file(logfile: &PathBuf) {
    info!("compress_file: {}", logfile.display());
    if !logfile.exists() {
        warn!("压缩前一天的日志文件不存在: {}", logfile.display());
        return;
    }

    if is_le_2k(logfile) {
        warn!("文件小于等于2K，不执行压缩操作");
        let new_path = PathBuf::from(format!("{}.log", logfile.display()));
        rename(logfile, &new_path);
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
    info!("压缩文件和删除旧的文件成功");
}

fn is_le_2k(file_path: &PathBuf) -> bool {
    let metadata = std::fs::metadata(file_path).unwrap();

    // 获取文件的大小，单位是字节
    let file_size = metadata.len();

    // 判断文件是否小于等于2K
    return file_size <= 2 * 1024;
}

fn rename(file_path: &PathBuf, new_path: &PathBuf) {
    std::fs::rename(file_path, new_path).unwrap()
}

fn is_le_00_30() -> bool {
    use chrono::{Local, Timelike};
    let now = Local::now();
    return now.hour() == 0 && now.minute() < 30;
}


#[test]
fn day_30() {
    use chrono::{Local, Timelike};

    let now = Local::now();

    if now.hour() == 0 && now.minute() < 30 {
        println!("现在的时间是00:30之前.");
    } else {
        println!("现在的时间是00:30之后.");
    }
}