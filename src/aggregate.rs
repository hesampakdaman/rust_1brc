use std::collections::HashMap;
use std::sync::mpsc::Receiver;

use crate::record::Record;

pub fn reduce(rx: Receiver<HashMap<String, Record>>) {
    let mut hmap = HashMap::new();
    while let Ok(stats) = rx.recv() {
        for (city, rec) in stats {
            hmap.entry(city).or_insert(Record::default()).merge(rec);
        }
    }
}
