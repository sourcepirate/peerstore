
use super::storage::{Storage, StorageResult};
use std::collections::HashMap;

pub struct MemoryStorage {
    inner: HashMap<String, String>
}

impl Storage<String> for MemoryStorage {
  
    fn put(&mut self, k: String, v: String) -> StorageResult<()> {
        match self.inner.insert(k, v) {
            Some(_) => Ok(()),
            None => Ok(())
        }
    }

    fn get(&self, k: String) -> Option<&String> {
        self.inner.get(&k)
    }
}

impl MemoryStorage {
    pub fn new() -> Self {
        MemoryStorage {
            inner: HashMap::new()
        }
    }
}