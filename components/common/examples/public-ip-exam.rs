/// 获取 Linux 服务器外网的 IP 地址
/// 要获取 Linux 服务器的外网 IP 地址，你可以使用 reqwest 库发送 HTTP 请求到一个公共的 IP 查询接口，并解析返回的数据来获取 IP 地址。
//
// 首先，在 Cargo.toml 文件中添加 reqwest 和 tokio 的依赖：
//
// toml
// [dependencies]
// reqwest = { version = "0.11", features = ["blocking"] }
// tokio = { version = "1", features = ["full"] }
// 然后，使用以下代码来获取 Linux 服务器的外网 IP 地址：
//
// rust
// use reqwest::blocking::Client;
// use std::net::IpAddr;
//
// fn main() {
//     let ip = get_public_ip().unwrap();
//     println!("Public IP Address: {}", ip);
// }
//
// fn get_public_ip() -> Result<IpAddr, Box<dyn std::error::Error>> {
//     let client = Client::new();
//     let response = client.get("https://api.ipify.org").send()?;
//     let ip = response.text()?;
//     let ip_addr: IpAddr = ip.trim().parse()?;
//
//     Ok(ip_addr)
// }
// 这段代码使用 reqwest::blocking::Client 来发送 GET 请求到 https://api.ipify.org 这个公共 IP 查询接口。然后，解析返回的数据并将其转换为 IpAddr 类型。
//
// 需要注意的是，这里使用了 reqwest 的 blocking 特性，以及 tokio 的 full 特性，因此需要在 Cargo.toml 文件中指定相应的版本号和特性。
//
// 现在，当你运行这段代码时，它将发送一个请求到 https://api.ipify.org 并返回 Linux 服务器的外网 IP 地址。
// 请注意，由于网络环境的不同，这个方法并不总是能够准确获取到外网 IP 地址。有时候，服务器可能位于 NAT 网络后面，或者无法直接访问外网，这种情况下可能无法获取到准确的外网 IP 地址。
fn main() {}