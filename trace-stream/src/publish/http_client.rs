use std::net::IpAddr;
use std::time::Duration;

use reqwest::{Client, ClientBuilder};
use reqwest::header::HeaderMap;

use common::log::{error, warn};

use crate::publish::errors::PubError;

pub struct ReqwestClient;

impl ReqwestClient {
    pub fn build() -> Client {
        Self.build_client(Self.client_builder())
    }

    pub fn build_headers(header: HeaderMap) -> Client {
        let builder = Self.client_builder().default_headers(header);
        Self.build_client(builder)
    }

    fn client_builder(&self) -> ClientBuilder {
        // let builder = match get_http_proxy() {
        //     Some(proxy) => Client::builder().proxy(proxy),
        //     None => Client::builder(),
        // };
        let builder = Client::builder();
        builder.timeout(Duration::from_secs(30))
    }

    fn build_client(&self, builder: ClientBuilder) -> Client {
        builder.build().unwrap_or_else(|err| {
            warn!("Build Reqwest Client error with default. \nerror: {:?}",err);
            Client::new()
        })
    }
}

pub async fn get_public_ip() -> Result<IpAddr, PubError> {
    let client = Client::new();
    let response = client.get("https://api.ipify.org").send().await?;
    let ip = response.text().await?;
    let ip_addr: IpAddr = ip.trim().parse()?;

    Ok(ip_addr)
}

pub async fn get_pub_ip_str() -> String {
    match get_public_ip().await {
        Ok(ip) => {
            ip.to_string()
        }
        Err(err) => {
            error!("Get public ip error: {:?}", err);
            "".to_string()
        }
    }
}

#[test]
fn test_get_pub_ip() {
    let ip = get_public_ip().unwrap();
    println!("Public IP Address: {ip}");
}

