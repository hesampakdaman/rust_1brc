use std::collections::HashMap;
use std::sync::mpsc::Receiver;

use crate::record::Record;

pub fn reduce(rx: Receiver<HashMap<String, Record>>) -> Vec<(String, Record)> {
    let mut hmap = HashMap::with_capacity(10_000);
    while let Ok(stats) = rx.recv() {
        for (city, record) in stats {
            hmap.entry(city)
                .and_modify(|existing: &mut Record| existing.merge(&record))
                .or_insert(record);
        }
    }
    let mut v = hmap.into_iter().collect::<Vec<_>>();
    v.sort_unstable_by(|a, b| a.0.cmp(&b.0));
    v
}
