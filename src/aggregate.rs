use std::collections::HashMap;
use std::sync::mpsc::Receiver;

use crate::record::Record;

pub fn reduce(rx: Receiver<HashMap<String, Record>>) -> Vec<(String, Record)> {
    let mut hmap: HashMap<String, Record> = HashMap::new();
    while let Ok(stats) = rx.recv() {
        for (city, new) in stats {
            hmap.entry(city)
                .and_modify(|exist| exist.merge(&new))
                .or_insert(new);
        }
    }
    let mut v = hmap.into_iter().collect::<Vec<_>>();
    v.sort_unstable_by(|a, b| a.0.cmp(&b.0));
    v
}
