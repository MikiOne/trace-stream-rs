// use std::sync::Arc;
// use tokio::fs::OpenOptions;
// use tokio::io::AsyncWriteExt;
// use tokio::sync::{mpsc, Mutex};
// use tokio::sync::mpsc::Receiver;
// use tokio::fs::File;
//
// pub async fn store(file_writer: Arc<Mutex<File>>, mut rx: Receiver<Vec<String>>) {
//     // 异步任务处理写入文件
//     let writer_clone = file_writer.clone();
//     tokio::spawn(async move {
//         while let Some(message) = rx.recv().await {
//             let mut file = writer_clone.lock().await;
//             file.write_all(message.as_bytes()).await.expect("");
//         }
//     }).await.expect("");
// }
//
//
//
// #[tokio::test]
// async fn test_append() -> Result<(), Box<dyn std::error::Error>> {
//     let (tx, rx) = mpsc::channel::<Vec<String>>(100);
//     let file_path = "/Users/egal/workspace/rust_ws/trace-stream-rs/trace-server/logs/oauth-server-2024-02-20.log";
//
//     // 打开文件一次，以追加模式写入
//     let file = OpenOptions::new()
//         .create(true)
//         .write(true)
//         .append(true)
//         .open(file_path)
//         .await?;
//
//     let writer = Arc::new(Mutex::new(file));
//
//     // 异步任务模拟数据产生
//     let tx_clone = tx.clone();
//     tokio::spawn(async move {
//         for i in 1..=99999999999999i64 {
//             let data = format!("这是第 {} 行数据.\n", i);
//             tx_clone.send(vec![data]).await.unwrap();
//             tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
//         }
//     });
//
//     store(writer, rx).await.unwrap();
//     Ok(())
// }
//
