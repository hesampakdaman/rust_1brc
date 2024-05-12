use std::hash::{Hash, Hasher};

#[derive(PartialEq, Eq)]
pub struct Key(u64);

impl Key {
    pub fn new(bytes: &[u8]) -> Self {
        // djb2 hash fn
        // hash(0) = 5381
        // hash(i) = hash(i-1) * 33 ^ byte[i]
        let hash_fn = |hash, byte: &u8| (hash * 33) ^ u64::from(*byte);
        Self(bytes.iter().fold(5381, hash_fn))
    }
}

impl Hash for Key {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u64(self.0);
    }
}
