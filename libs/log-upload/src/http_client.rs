use std::net::IpAddr;
use std::time::Duration;

use reqwest::{Client, ClientBuilder};
use reqwest::blocking;
use reqwest::header::HeaderMap;

use common::log::{error, warn};

use crate::errors::PubError;

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

pub fn get_public_ip() -> Result<IpAddr, PubError> {
    let client = blocking::Client::new();
    let response = client.get("https://api.ipify.org").send()?;
    let ip = response.text()?;
    let ip_addr: IpAddr = ip.trim().parse()?;

    Ok(ip_addr)
}

pub fn get_pub_ip_str() -> String {
    match get_public_ip() {
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

// pub fn get_http_proxy() -> Option<Proxy> {
//     let load_http_proxy = |proxy_cfg: ProxyConfig| {
//         let proxies = proxy_cfg.proxies;
//         if proxies.len() > 0 {
//             match proxies.get("http") {
//                 Some(val) => Some(val.to_string()),
//                 None => None,
//             }
//         } else {
//             None
//         }
//     };
//
//     let http_proxy_opt = |cfg_opt: Option<ProxyConfig>| match cfg_opt {
//         Some(proxy_config) => {
//             debug!("get_local_proxy config: {:?}", proxy_config);
//             load_http_proxy(proxy_config)
//         }
//         None => None,
//     };
//
//     let proxy_ip_port = match proxy_cfg::get_proxy_config() {
//         Ok(cfg_opt) => http_proxy_opt(cfg_opt),
//         Err(err) => {
//             warn!("proxy_cfg::get_proxy_config error: {:?}", err);
//             None
//         }
//     };
//
//     let build_http_proxy = |proxy_ip_port| match Proxy::http(proxy_ip_port) {
//         Ok(proxy) => {
//             // info!("Build reqwest http proxy: {:?}", &proxy);
//             Some(proxy)
//         }
//         Err(err) => {
//             warn!("Build reqwest http Proxy error: {:?}", err);
//             None
//         }
//     };
//
//     match proxy_ip_port {
//         Some(proxy_ip_port) => build_http_proxy(proxy_ip_port),
//         None => None,
//     }
// }
