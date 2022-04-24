use std::io::Write;
use std::net::{SocketAddrV4, TcpStream};

pub enum RingError {
    BroadCastFailure,
    Unknown,
}

pub struct RingNetwork {
    peers: Vec<SocketAddrV4>,
}

impl RingNetwork {
    pub fn new() -> Self {
        RingNetwork { peers: Vec::new() }
    }

    pub fn join(&mut self, addr: SocketAddrV4) {
        self.peers.push(addr)
    }

    pub fn broadcast(&self, msg: &str) -> Result<(), RingError> {
        for ip in self.peers.iter() {
            if let Ok(mut stream) = TcpStream::connect(ip) {
                match stream.write(msg.as_bytes()) {
                    Ok(sz) => println!("Wrote {:?} bytes", sz),
                    Err(_) => {}
                };
            } else {
                return Err(RingError::BroadCastFailure);
            }
        }
        Ok(())
    }
}
