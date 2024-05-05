use std::collections::HashMap;
use std::sync::mpsc::Sender;
use memmap2::MmapOptions;
use crate::pre_processing::Chunk;
use crate::record::Record;

pub fn stats(chunk: Chunk, tx: Sender<HashMap<String, Record>>) {
    let file = std::fs::File::open("./measurements.txt").unwrap();
    let mmap = unsafe { MmapOptions::new().map(&file).unwrap() };
    let start = chunk.offset as usize;
    let end = start + chunk.size as usize;
    let segment = &mmap[start..end];
    let mut map: HashMap<Vec<u8>, Record> = HashMap::new();
    for line in segment.split(|&b| b == b'\n') {
        if !line.is_empty() {
            let mut splitted = line.split(|&b| b == b';');
            let city = splitted.next().unwrap();
            let float = parse_float(splitted.next().unwrap());
            if let Some(rec) = map.get_mut(city) {
                rec.add(float);
            } else {
                map.insert(city.to_vec(), Record::from(float));
            }
        }
    }
    // tx.send(statistics.0).unwrap();
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
