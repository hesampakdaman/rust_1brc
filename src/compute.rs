use crate::record::Record;
use fxhash::FxHashMap;
use memchr::memchr;
use std::sync::mpsc::Sender;

#[derive(Hash, PartialEq, Eq)]
pub struct CityKey(u64);

impl CityKey {
    fn djb2(bytes: &[u8]) -> u64 {
        let mut hash = 5381;
        for byte in bytes {
            hash = ((hash << 5) + hash) + u64::from(*byte); // hash * 33 + c
        }
        hash
    }
}

impl From<&[u8]> for CityKey {
    fn from(bytes: &[u8]) -> Self {
        let hash = Self::djb2(bytes);
        Self(hash)
    }
}

pub fn stats(bytes: &[u8], tx: Sender<FxHashMap<CityKey, Record>>) {
    let hmap = calculate(bytes);
    tx.send(hmap).unwrap();
}

fn calculate(mut bytes: &[u8]) -> FxHashMap<CityKey, Record> {
    let mut map: FxHashMap<CityKey, Record> = FxHashMap::default();
    while let Some(sep_idx) = memchr(b';', bytes) {
        let end_idx = memchr(b'\n', bytes).unwrap_or(bytes.len());
        let key = CityKey::from(&bytes[..sep_idx]);
        let num = parse_float(&bytes[sep_idx + 1..end_idx]);
        if let Some(rec) = map.get_mut(&key) {
            rec.add(num);
        } else {
            let name = unsafe { std::str::from_utf8_unchecked(&bytes[..sep_idx]) };
            map.insert(key, Record::from((name, num)));
        }
        bytes = if end_idx < bytes.len() {
            &bytes[end_idx + 1..]
        } else {
            &[]
        };
    }
    map
}

fn parse_float(bytes: &[u8]) -> i32 {
    let mut result = 0;
    let mut is_negative = false;
    for &b in bytes {
        match b {
            b'0'..=b'9' => {
                let digit = (b - b'0') as i32;
                result = result * 10 + digit;
            }
            b'-' => is_negative = true,
            _ => {}
        }
    }
    if is_negative {
        result *= -1;
    }
    result
}
