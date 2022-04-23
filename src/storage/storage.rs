use std::hash::Hash;

pub enum StorageError {
    InSufficientSpace,
    UnknownError
}

pub type StorageResult<T> = Result<T, StorageError>;

pub trait Storage<T: Hash> {
    fn put(&mut self, key: T, value:T) -> StorageResult<()>;
    fn get(&self, key: T) -> Option<&T>;
}

