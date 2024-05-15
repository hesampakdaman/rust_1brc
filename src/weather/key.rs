use std::hash::{Hash, Hasher};

#[derive(PartialEq, Eq)]
pub struct Key(u64);

impl Key {
    pub fn new(bytes: &[u8]) -> Self {
        // djb2 hash fn
        // hash(0) = 5381
        // hash(i) = hash(i-1) * 33 ^ byte[i]
        let hash_fn = |hash: u64, &byte| hash.wrapping_mul(33) + byte as u64;

        let len = bytes.len();
        let first = &bytes[..len.min(8)];
        let last = if len > 8 { &bytes[len - 8..] } else { &[] };
        Self(
            first
                .iter()
                .chain(last)
                .fold(5381, hash_fn)
                .wrapping_mul(len as u64), // multiply with the length of input to ensure no collisions
        )
    }
}

impl Hash for Key {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u64(self.0);
    }
}
