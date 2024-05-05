use crate::pre_processing::Chunk;
use crate::record::Record;
use memmap2::MmapOptions;
use std::collections::HashMap;
use std::sync::mpsc::Sender;

pub fn stats(chunk: Chunk, tx: Sender<HashMap<String, Record>>) {
    let file = std::fs::File::open("./measurements.txt").unwrap();
    let mmap = unsafe { MmapOptions::new().map(&file).unwrap() };
    let map = compute(&mmap[chunk.offset as usize..(chunk.offset+chunk.size) as usize]);
    tx.send(map).unwrap();
}

fn compute(bytes: &[u8]) -> HashMap<String, Record> {
    let mut map: HashMap<String, Record> = HashMap::with_capacity(10_000);
    for line in bytes.split(|&b| b == b'\n') {
        if !line.is_empty() {
            let mut splitted = line.split(|&b| b == b';');
            let city = unsafe { std::str::from_utf8_unchecked(splitted.next().unwrap()) };
            let float = parse_float(splitted.next().unwrap());
            if let Some(rec) = map.get_mut(city) {
                rec.add(float);
            } else {
                map.insert(city.to_string(), Record::from(float));
            }
        }
    }
    map
}

fn parse_float(bytes: &[u8]) -> i32 {
    let mut result = 0;
    let mut is_positive = true;
    for &b in bytes {
        match b {
            b'0'..=b'9' => {
                let digit = (b - b'0') as i32;
                result = result * 10 + digit;
            }
            b'-' => {is_positive = false},
            _ => {}
        }
    }
    if is_positive { result} else { -result }
}
