use std::net::{Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};
use std::process::Command;

pub fn get_host_ip(port: u16) -> Option<SocketAddr> {
    let output = match Command::new("hostname").args(&["-I"]).output() {
        Ok(ok) => ok,
        Err(_) => {
            return None;
        }
    };

    let stdout = match String::from_utf8(output.stdout) {
        Ok(ok) => ok,
        Err(_) => {
            return None;
        }
    };

    let ips: Vec<&str> = stdout.trim().split(" ").collect::<Vec<&str>>();
    let first = ips.first();
    match first {
        Some(first) => {
            if !first.is_empty() {
                if let Ok(addr) = first.parse::<Ipv4Addr>() {
                    return Some(SocketAddr::V4(SocketAddrV4::new(addr, port)));
                } else if let Ok(addr) = first.parse::<Ipv6Addr>() {
                    return Some(SocketAddr::V6(SocketAddrV6::new(addr, port, 0, 0)));
                } else {
                    None
                }
            } else {
                None
            }
        }
        None => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_if_ip_construction_works() {
        let port : u16 = 8080;
        let res = get_host_ip(port);
        println!("res: {:?}", res);
        assert!(res.is_some());
    }
}