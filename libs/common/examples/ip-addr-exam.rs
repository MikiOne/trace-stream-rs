use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};

fn main() {
    // 获取本机 IP v4 地址
    let ip_v4 = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));
    let socket_v4 = SocketAddr::new(ip_v4, 0);
    let local_v4 = std::net::UdpSocket::bind(socket_v4).unwrap().local_addr().unwrap().ip();
    println!("本机 IP v4 地址为: {}", local_v4);

    // 获取本机 IP v6 地址
    let ip_v6 = IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0));
    let socket_v6 = SocketAddr::new(ip_v6, 0);
    let local_v6 = std::net::UdpSocket::bind(socket_v6).unwrap().local_addr().unwrap().ip();
    println!("本机 IP v6 地址为: {}", local_v6);
}
