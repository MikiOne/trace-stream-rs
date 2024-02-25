use std::fs;
use std::fs::{DirEntry, File, Metadata, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Utc};
use log::{error, info, warn};
// use regex::{Match, Regex};


pub fn exists(file_buf: &PathBuf) -> bool {
    file_buf.exists()
}

pub fn create_dir(dir_path: &Path) {
    if !dir_path.exists() {
        let dir = dir_path.to_str().unwrap();
        match fs::create_dir_all(dir_path) {
            Ok(_) => info!("dir {:?} created", dir_path.to_str()),
            Err(error) => error!("create {} dir error: {}", dir, error),
        }
    }
}

pub fn create_app_file(file_path: PathBuf) {
    let prefix = file_path.parent().unwrap();
    create_dir(prefix);

    if !file_path.exists() {
        let file_path_c = file_path.clone();
        match OpenOptions::new().create(true).append(true).open(file_path) {
            Ok(file) => info!("file {:?} created", file),
            Err(e) => error!(
                "create {} file error: {}",
                file_path_c.file_name().unwrap().to_str().unwrap(),
                e
            ),
        }
    }
}

pub fn load_file_entries(folder_path: &PathBuf) -> Vec<DirEntry> {
    match fs::read_dir(&folder_path) {
        Ok(files) => files
            .filter_map(|file| file.ok())
            .collect::<Vec<DirEntry>>(),
        Err(err) => {
            error!("load folder[{:?}] entries err: {:?}", folder_path, err);
            Vec::new()
        }
    }
}

// pub fn is_new_file(file_path: &PathBuf) -> bool {
//     // let metadata = fs::metadata(&file_path)?;
//     // let created = metadata.created()?;
//
//     let compare = |metadata: Metadata| {
//         let created = metadata.created().unwrap_or(SystemTime::now());
//         let modified = metadata.modified().unwrap_or(SystemTime::now());
//
//     };
//
//     match fs::metadata(&file_path) {
//         Ok(md) => {
//
//
//         }
//         Err(err) => {
//            warn!("Get file[{:?}] metadata error: {:?}", file_path, err);
//             false
//         }
//     }
//
// }

// pub fn get_log_file_update_time(entry: &DirEntry) -> DateTime<Utc> {
//     match entry.metadata() {
//         Ok(metadata) => {
//             let sys_time = metadata.modified().unwrap();
//             DateTime::from(sys_time)
//         }
//         Err(err) => {
//             warn!("get_log_file_create_time error: {:?}", err);
//             let filename = entry.file_name().into_string().unwrap();
//             get_log_file_time(filename)
//         }
//     }
// }

// pub fn get_log_file_time(filename: String) -> DateTime<Utc> {
//     let parse_time = |date_str: String| {
//         let year = &date_str[0..4];
//         let month = &date_str[4..6];
//         let day = &date_str[6..8];
//
//         let naive_date =
//             NaiveDate::parse_from_str(&format!("{}-{}-{}", year, month, day), "%Y-%m-%d").unwrap();
//         let naive_datetime =
//             NaiveDateTime::new(naive_date, NaiveTime::from_hms_opt(0, 0, 0).unwrap());
//         Utc.from_local_datetime(&naive_datetime).unwrap()
//     };
//
//     let date_regex = Regex::new(r"\d{8}").unwrap();
//     match date_regex.find(&filename) {
//         Some(date_str) => {
//             let date_str = filename[date_str.start()..date_str.end()].to_string();
//             parse_time(date_str)
//         }
//         None => {
//             warn!("get_log_file_time by filename, filename time string is none, return DateTime::default");
//             DateTime::from(SystemTime::now())
//         }
//     }
// }


#[test]
fn test_create_file_truncate() -> std::io::Result<()> {
    let mut f = File::create("foo.txt")?;
    f.write_all(&1234_u32.to_be_bytes())?;
    Ok(())
}

#[test]
fn test_open_file_write() -> std::io::Result<()> {
    let file_path = "/Users/egal/workspace/rust_ws/trace-stream-rs/trace-server/logs/foo.txt";
    // 使用 OpenOptions 打开文件
    let mut file = OpenOptions::new()
        .append(true) // 设置追加模式
        .open(file_path)
        .expect("无法打开文件");

    // 要写入的数据
    let data = "一些新的数据\n";

    // 写入数据到文件
    file.write_all(data.as_bytes())
        .expect("无法写入文件");
    Ok(())
}
