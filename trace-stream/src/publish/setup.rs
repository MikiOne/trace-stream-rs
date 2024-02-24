use tokio::sync::OnceCell;
use crate::publish::api_upload::login;

use crate::publish::settings::RemoteServerConfig;

pub static REMOTE_SERVER: OnceCell<RemoteServerConfig> = OnceCell::const_new();


pub async fn init(config: RemoteServerConfig) {
    store_remote_server_config(config).await;
    login().await;
}

async fn store_remote_server_config(config: RemoteServerConfig) {
    REMOTE_SERVER.get_or_init(|| async { config }).await;
}