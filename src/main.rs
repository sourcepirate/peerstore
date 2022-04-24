//!
//!
//!
use std::io::Read;
use std::net::{TcpListener, TcpStream};

pub mod command;
pub mod network;
pub mod peerstore;
pub mod storage;

use command::commander::CommandType;
use peerstore::PeerStore;

fn main() {
    let mut store = PeerStore::new();
    let listener = TcpListener::bind("0.0.0.0:4040").unwrap();
    for stream in listener.incoming() {
        let mut intr: TcpStream = stream.unwrap();
        let mut value: String = String::new();
        if let Ok(_) = intr.read_to_string(&mut value) {
            let command_type: CommandType = command::commander::resolve(value.clone());
            match command_type {
                CommandType::NetworkCommand => {
                    store.network_command(value.into());
                    println!("Network command");
                }
                CommandType::StorageCommand => {
                    store.storage_command(value.into());
                    println!("Storage command");
                }
                CommandType::UnknownCommand => {
                    println!("Unknown command");
                }
            }
        }
    }
}
