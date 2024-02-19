use get_if_addrs::get_if_addrs;

fn if_addrs() {
    if let Ok(if_addrs) = get_if_addrs() {
        if_addrs.iter().for_each(|iface| {
            match &iface.addr {
                get_if_addrs::IfAddr::V4(sad) => {
                    println!("IPv4 address: {}", sad.ip);
                }
                get_if_addrs::IfAddr::V6(sad) => {
                    println!("IPv6 address: {}", sad.ip);
                }
            }
        });
    } else {
        println!("Failed to get network interfaces.");
    }
}

fn get_if_addrs_test() {
    let ifaces = get_if_addrs::get_if_addrs().unwrap();
    println!("Got list of interfaces");
    println!("{:#?}", ifaces);
}

fn main() {
    get_if_addrs_test();
    if_addrs();
}
