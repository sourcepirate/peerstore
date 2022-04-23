use std::io::Write;
use std::net::{TcpStream, SocketAddrV4};



pub struct RingNetwork {
    peers: Vec<SocketAddrV4>,
}

impl RingNetwork {

    pub fn new() -> Self {
        RingNetwork {
            peers: Vec::new()
        }
    }

    pub fn join(&mut self, addr: SocketAddrV4) {
        self.peers.push(addr)
    }

    pub fn broadcast(&self, msg: &str) {
        for ip in self.peers.iter() {
           if let Ok(mut stream) = TcpStream::connect(ip) {
               stream.write(msg.as_bytes());
           } else {
               println!("cannot broadcast message to {}", ip);
           }
        }
    }
}