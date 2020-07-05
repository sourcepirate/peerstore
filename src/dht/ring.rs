use super::consistent::{inrange, CHash};
use super::ip::get_host_ip;
use serde::{Deserialize, Serialize};
use std::io;
use std::io::{Read, Write};
use std::net::SocketAddr;
use std::net::TcpStream;
use rmp_serde::{Serializer, Deserializer};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum DHTMessage {
    FindSuccessor(SocketAddr, CHash),
    Node(SocketAddr, CHash),
    None
}

pub trait Transport {
    fn send(&self, sock: SocketAddr, message: DHTMessage) -> io::Result<DHTMessage>;
}

#[derive(Debug)]
pub struct DHTRing<T: Transport> {
    fingers: Vec<SocketAddr>,
    id: CHash,
    predecessor: Option<CHash>,
    successor: Option<CHash>,
    addr: SocketAddr,
    transport: T,
}

impl<T: Transport> DHTRing<T> {
    pub fn new(port: u16, transport: T) -> Self {
        let socket_addr = get_host_ip(port).unwrap();
        let hash = CHash::new(socket_addr);
        DHTRing {
            fingers: Vec::new(),
            id: hash,
            predecessor: None,
            successor: None,
            addr: socket_addr,
            transport,
        }
    }

    pub fn join(&mut self, node: SocketAddr) -> io::Result<()> {
        self.predecessor = None;
        let dht_opt = self
            .transport
            .send(node, DHTMessage::FindSuccessor(node, self.id.clone()))?;
            match dht_opt {
                DHTMessage::Node(_, hash_id) => {
                    self.successor = Some(hash_id);
                    Ok(())
                }
                _ => Ok(()),
            }

    }

    pub fn find_successor(&mut self, node: CHash) -> Option<CHash> {
        if self.successor.is_some() {
            let successor_value: CHash = self.successor.as_ref().map(|x: &CHash| *x).unwrap();
            if inrange(&node, &self.id, &successor_value) {
                return Some(successor_value);
            } else {
                let (_, address) = self.closest_peer(node);
                let dht_res = self
                    .transport
                    .send(address, DHTMessage::FindSuccessor(address, node));
                if dht_res.is_ok() {
                    let dht_opt = dht_res.unwrap();
                    if let DHTMessage::Node(_, hash_id) = dht_opt {
                        return Some(hash_id);
                    }
                }
                None
            }
        } else {
            None
        }
    }

    pub fn address(&self) -> SocketAddr {
        self.addr
    }

    pub fn closest_peer(&mut self, node: CHash) -> (CHash, SocketAddr) {
        for adj_node in self.fingers.iter() {
            let hash = CHash::new(adj_node);
            if inrange(&hash, &self.id, &node) {
                return (hash, adj_node.clone());
            }
        }
        (self.id, self.addr.clone())
    }
}

#[derive(Debug, Clone, Copy)]
struct TcpTransport;

impl Transport for TcpTransport {
    fn send(&self, sock: SocketAddr, msg: DHTMessage) -> io::Result<DHTMessage> {
        let mut buffer = Vec::new();
        msg.serialize(&mut Serializer::new(&mut buffer)).unwrap();
        let mut stream = TcpStream::connect(sock)?;
        stream.write(&buffer)?;
        let mut deserializer = Deserializer::new(stream);
        match Deserialize::deserialize(&mut deserializer) {
            Ok(msg) => Ok(msg),
            Err(_) => Ok(DHTMessage::None)
        }
    }
}
