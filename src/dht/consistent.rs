use siphasher::sip::SipHasher;
use std::cmp::Ordering;
use std::hash::{BuildHasher, Hash, Hasher};

pub struct DefaultClusterHash;

impl BuildHasher for DefaultClusterHash {
    type Hasher = SipHasher;

    fn build_hasher(&self) -> Self::Hasher {
        SipHasher::new()
    }
}

pub fn get_hash<S, T>(hash_builder: &S, input: T) -> u64
where
    S: BuildHasher,
    T: Hash,
{
    let mut hasher = hash_builder.build_hasher();
    input.hash(&mut hasher);
    let hash = hasher.finish();

    let buf = hash.to_be_bytes();

    u64::from(buf[7]) << 56
        | u64::from(buf[6]) << 48
        | u64::from(buf[5]) << 40
        | u64::from(buf[4]) << 32
        | u64::from(buf[3]) << 24
        | u64::from(buf[2]) << 16
        | u64::from(buf[1]) << 8
        | u64::from(buf[0])
}

pub struct CHash(u64);

impl CHash {
    pub fn new<U: Hash>(val: U) -> Self {
        let hasher: DefaultClusterHash = DefaultClusterHash;
        let value: u64 = get_hash(&hasher, val);
        CHash(value)
    }
}

impl PartialEq for CHash {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for CHash {}

impl PartialOrd for CHash {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl Ord for CHash {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}


