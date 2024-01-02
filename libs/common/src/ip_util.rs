use pnet::datalink::{self, NetworkInterface};
use std::net::IpAddr;
use log::debug;

fn get_ipv4() -> Option<IpAddr> {
    let interfaces = datalink::interfaces();

    for interface in interfaces {
        if interface.is_loopback() {
            continue;
        }

        if let Some(ipv4) = get_ipv4_address(&interface) {
            debug!("IPv4 Address: {}", ipv4);
            return Some(ipv4);
        }
    }

    None
}

fn get_ipv4_address(interface: &NetworkInterface) -> Option<IpAddr> {
    for ip_network in &interface.ips {
        if let IpAddr::V4(ipv4) = ip_network.ip() {
            if !ipv4.is_loopback() {
                return Some(IpAddr::from(ipv4));
            }
        }
    }

    None
}
