use serde::Deserialize;
use tokio::sync::OnceCell;

pub mod auth;
pub mod middleware;

static STATIC_OAUTH: OnceCell<StaticOauth> = OnceCell::const_new();

pub async fn init_static_oauth(oauth: &StaticOauth) {
    STATIC_OAUTH.get_or_init(|| async { oauth.to_owned() }).await;
}


#[derive(Debug, Clone, Deserialize)]
#[allow(unused)]
pub struct StaticOauth {
    pub auth_uid: String,
    pub pwd_md5: String,
    pub pwd_bcrypt_hash: String,
}