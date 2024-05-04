use std::collections::HashMap;
use std::io::{Read, Seek};
use std::sync::mpsc::Sender;

use crate::pre_processing::Chunk;
use crate::record::Record;

pub fn stats(chunk: Chunk, tx: Sender<HashMap<String, Record>>) {
    let f = std::fs::File::open("./measurements.txt").unwrap();
    let mut reader = std::io::BufReader::new(f);
    let mut buf = vec![0; chunk.size as usize];
    reader.seek(std::io::SeekFrom::Start(chunk.offset)).unwrap();
    reader.read_exact(&mut buf).unwrap();
    let mut statistics = Statistics(HashMap::new());
    for line in buf.split(|&b| b == b'\n') {
        if !line.is_empty() {
            let splitted: Vec<_> = line.split(|&b| b == b';').collect();
            let city = unsafe { std::str::from_utf8_unchecked(splitted[0]) };
            let float = parse_float(splitted[1]);
            statistics.add(city.to_string(), float);
        }
    }
    tx.send(statistics.0).unwrap();
}

fn parse_float(bytes: &[u8]) -> f32 {
    let mut result = 0.0;
    let mut divisor = 1.0;
    let mut is_fraction = false;

    for &b in bytes {
        match b {
            b'0'..=b'9' => {
                let digit = (b - b'0') as f32;
                if is_fraction {
                    divisor *= 10.0;
                    result += digit / divisor;
                } else {
                    result = result * 10.0 + digit;
                }
            }
            b'.' => {
                is_fraction = true;
            }
            _ => {} // Handle unexpected characters or simply ignore based on the assumption of valid input
        }
    }

    result
}

struct Statistics(HashMap<String, Record>);

impl Statistics {
    fn add(&mut self, city: String, t: f32) {
        if let Some(rec) = self.0.get_mut(&city) {
            rec.add(t);
        } else {
            self.0.insert(city.to_string(), Record::from(t));
        };
    }
}
