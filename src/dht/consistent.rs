use siphasher::sip::SipHasher;
use std::cmp::Ordering;
use std::hash::{BuildHasher, Hash, Hasher};

pub const RANGE_SIZE : u64 = u64::MAX;

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


#[derive(Clone, Debug)]
pub struct CHash(u64);

impl CHash {
    pub fn new<U: Hash>(val: U) -> Self {
        let hasher: DefaultClusterHash = DefaultClusterHash;
        let value: u64 = get_hash(&hasher, val);
        CHash(value % RANGE_SIZE)
    }

    pub fn min() -> Self {
        CHash(0)
    }
    
    pub fn max() -> Self {
        CHash(RANGE_SIZE - 1)
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


pub fn inrange(c: &CHash, a: &CHash, b: &CHash)  -> bool {
   if a <= b {
       if c >= a && c < b {
           return true
       } else {
           return false
       }
   }
   else {
       false
   }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_consistent_compare() {
        let c1 : CHash = CHash::new("abc");
        let c2 : CHash = CHash::new("abc");

        assert_eq!(c1, c2);
    }


    #[test]
    fn test_consistent_range() {
        let c1 : CHash = CHash::new("abc");
        assert!(inrange(&c1, &CHash::min(), &CHash::max()))
    }


}