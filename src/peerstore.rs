use crate::command::storage::StorageCommand;
use crate::network::ring::RingNetwork;
use crate::storage::memory::MemoryStorage;
use crate::storage::storage::Storage;
use std::net::SocketAddrV4;

pub struct PeerStore {
    storage: MemoryStorage,
    network: RingNetwork,
}

impl PeerStore {
    pub fn new() -> Self {
        PeerStore {
            storage: MemoryStorage::new(),
            network: RingNetwork::new(),
        }
    }

    pub fn join(&mut self, socket: SocketAddrV4) {
        self.network.join(socket)
    }

    pub fn storage_command(&mut self, command: StorageCommand) -> Option<String> {
        match command {
            StorageCommand::Put(key, value) => {
                println!("Put {} => {}", key, value);
                self.storage.put(key.clone(), value);
                Some(key.clone())
            }
            StorageCommand::Get(key) => {
                println!("Get {}", key);
                let value = self.storage.get(key.clone()).unwrap();
                Some(value.to_owned())
            }
            StorageCommand::Delete(key) => {
                println!("Delete {}", key);
                Some(key.clone())
            }
        }
    }
}
