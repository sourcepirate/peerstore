//! 
//! 
//! 
use std::net::SocketAddrV4;
use std::net::{TcpListener, TcpStream};
use std::io::Read;


pub mod network;
pub mod storage;
pub mod command;
pub mod peerstore;

use peerstore::PeerStore;

fn main() {
    let addr : SocketAddrV4 = "0.0.0.0:8080".parse().expect("unable to parse");
    let mut store = PeerStore::new();
    let listener = TcpListener::bind("0.0.0.0:4040").unwrap();
    for stream in listener.incoming(){
        let mut intr : TcpStream = stream.unwrap();
        let mut value : String = String::new() ;
        intr.read_to_string(&mut value);
        println!("{}", value);
    }
}
