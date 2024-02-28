use std::sync::{Arc};
use tokio::{
    fs::{OpenOptions, File},
    io::{AsyncWriteExt},
    sync::mpsc,
};
use tokio::sync::Mutex;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (tx, mut rx) = mpsc::channel::<String>(100);
    let file_path = "/Users/egal/workspace/rust_ws/trace-stream-rs/trace-server/logs/oauth-server-2024-02-20.log";

    // 打开文件一次，以追加模式写入
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(file_path)
        .await?;

    let writer = Arc::new(Mutex::new(file));

    // 异步任务模拟数据产生
    let tx_clone = tx.clone();
    tokio::spawn(async move {
        for i in 1..=99999999999999i64 {
            let data = format!("这是第 {} 行数据.\n", i);
            tx_clone.send(data).await.unwrap();
            tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
        }
    });

    // 异步任务处理写入文件
    let writer_clone = writer.clone();
    tokio::spawn(async move {
        while let Some(message) = rx.recv().await {
            let mut file = writer_clone.lock().await;
            file.write_all(message.as_bytes()).await.unwrap();
        }
    }).await?;

    Ok(())
}

// 在这里我们使用了Arc和Mutex，因为我们需要在async块内共享和修改File。
// Arc使我们能够在线程或任务间共享File，而Mutex确保每次只有一个任务可以写入File。