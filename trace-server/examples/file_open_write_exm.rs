use tokio::{
    fs::OpenOptions,
    io::AsyncWriteExt,
    sync::mpsc,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (tx, mut rx) = mpsc::channel::<Vec<u8>>(100);
    let file_path = "/Users/egal/workspace/rust_ws/trace-stream-rs/trace-server/logs/oauth-server-2024-02-20.log";

    // 产生数据的异步任务
    tokio::spawn(async move {
        for i in 1..=9999999 {
            let data = format!("这是第 {} 行数据.\n", i);
            tx.send(data.into_bytes()).await.unwrap();
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
    });

    // 文件追加写入操作的异步任务
    tokio::spawn(async move {
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(file_path)
            .await
            .unwrap();

        while let Some(data) = rx.recv().await {
            // let mut writer = file.clone();
            file.write_all(&data).await.unwrap();
        }
    })
        .await?;

    Ok(())
}