// use std::fs::{File, OpenOptions};
// use std::io::{self, BufWriter, Write};
// use std::path::PathBuf;
//
// use flate2::Compression;
// use flate2::write::GzEncoder;
// use log::{debug, error, warn};
// use regex::Regex;
// use tokio::sync::OnceCell;
//
// use common::data_utils::to_previous_day;
// use common::file_utils::create_dir;
// use common::models::LogBody;
//
// pub static STORE_PATH: OnceCell<PathBuf> = OnceCell::const_new();
//
// pub async fn init_store_path(path: &PathBuf) {
//     STORE_PATH.get_or_init(|| async { path.to_owned() }).await;
// }
//
// fn starts_with_date(s: &str) -> bool {
//     let date_regex = Regex::new(r"^\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}").unwrap();
//     let date_s_regex = Regex::new(r"^\[\d{4}-\d{2}-\d{2}").unwrap();
//     let date_t_regex = Regex::new(r"^\[\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}").unwrap();
//     date_regex.is_match(s) || date_s_regex.is_match(s) || date_t_regex.is_match(s)
// }
//
// pub fn store(log: &LogBody) {
//     let store_path = STORE_PATH.get().unwrap();
//     debug!("日志存储路径: {}", store_path.display());
//     let store_path = store_path.join(format!("{}/", log.project_name));
//
//     debug!("日志存储目录: {}", store_path.display() );
//     // 创建目录
//     create_dir(store_path.as_path());
//
//     let filename = format!("{}-{}.log", log.server_name, log.log_day);
//     debug!("日志文件: {}", &filename);
//     let logfile = store_path.join(filename);
//
//     let header = format!("[{}:{}]", log.server_ip, log.server_name);
//     let lines = log.log_info.split("\n");
//     let log_lines: Vec<String> = lines
//         .map(|line| if starts_with_date(line)
//         { format!("{} {}", header, line) } else { line.to_string() })
//         .collect();
//
//     if logfile.exists() {
//         write_with_append(&logfile, log_lines);
//     } else {
//         write_with_create(&logfile, log_lines);
//
//         // 如果创建新的文件，则需要压缩旧的文件
//         compress_old_file(log);
//     }
// }
//
// /// 已追加的方式写入数据
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
//
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
//
// fn writeln(file: File, log_lines: Vec<String>) {
//     let mut writer = BufWriter::new(file);
//     for line in log_lines {
//         if line.len() > 0 {
//             writeln!(writer, "{}", line).unwrap();
//         }
//     }
//     writer.flush().unwrap();
// }
//
// fn compress_old_file(log: &LogBody) {
//     let store_path = STORE_PATH.get().unwrap()
//         .join(format!("{}/", log.project_name));
//     let pre_day = to_previous_day(log.log_day.as_str());
//     match pre_day {
//         Some(pre_day) => {
//             let old_logfile = store_path.join(format!("{}-{}.log", log.server_name, pre_day));
//             compress_file(&old_logfile);
//         }
//         None => {
//             warn!("无法获取前一天的日期: {}", log.log_day)
//         }
//     }
// }
//
// fn compress_file(logfile: &PathBuf) {
//     if !logfile.exists() {
//         warn!("压缩前一天的日志文件不存在: {}", logfile.display());
//         return;
//     }
//
//     let output_path = format!("{}.gz", logfile.display());
//     // 打开待压缩的文件
//     let mut input_file = File::open(logfile).expect("打开待压缩的文件");
//
//     // 创建一个写文件操作，以写入压缩的数据
//     let output_file = File::create(output_path).expect("创建压缩文件失败");
//
//     // 创建Gz编码器，用于压缩数据。
//     let mut encoder = GzEncoder::new(output_file, Compression::default());
//
//     // 将内容复制到编码器中，它会将压缩后的数据写入到输出文件
//     io::copy(&mut input_file, &mut encoder).expect("压缩失败");
//
//     // 完成压缩并刷新任何剩余的输出
//     encoder.finish().expect("压缩失败");
//
//     // 删除原文件
//     std::fs::remove_file(logfile).expect("删除文件失败");
// }
//
//
// #[test]
// fn test_compress_file() {
//     let input_path = "./logs/default.log-20240222-0.log";  // 输入文件
//     let input_path = PathBuf::from(input_path);
//     // 对文件进行压缩
//     compress_file(&input_path);
// }