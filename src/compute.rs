use std::collections::HashMap;
use std::io::{BufReader, Read, Seek};
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
    for line in String::from_utf8(buf).unwrap().trim_end().split('\n') {
        statistics.add(line);
    }
    tx.send(statistics.0).unwrap();
}

struct Statistics(HashMap<String, Record>);

impl Statistics {
    fn add(&mut self, line: &str) {
        let cr: CityRecord = line.into();
        if let Some(rec) = self.0.get_mut(&cr.city) {
            rec.merge(cr.temprature);
        } else {
            self.0.insert(cr.city, Record::from(cr.temprature));
        };
    }
}

struct CityRecord {
    city: String,
    temprature: f32,
}

impl From<&str> for CityRecord {
    fn from(value: &str) -> Self {
        let split = value.split(';').collect::<Vec<_>>();
        if split.len() < 2 {
            println!("{}", value);
        }
        Self { city: split[0].to_string(), temprature: split[1].trim().parse().unwrap() }
    }
}

