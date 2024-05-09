use crate::record::Record;
use std::collections::HashMap;
use std::sync::mpsc::Receiver;

pub fn reduce(rx: Receiver<HashMap<String, Record>>) -> Vec<(String, Record)> {
    let mut hmap = HashMap::with_capacity(10_000);
    while let Ok(stats) = rx.recv() {
        merge_records(&mut hmap, stats);
    }
    to_sorted_vec(hmap)
}

fn merge_records(dst: &mut HashMap<String, Record>, src: HashMap<String, Record>) {
    for (city, new_record) in src {
        dst.entry(city)
            .and_modify(|existing_record: &mut Record| existing_record.merge(&new_record))
            .or_insert(new_record);
    }
}

fn to_sorted_vec(hmap: HashMap<String, Record>) -> Vec<(String, Record)> {
    let mut v = hmap.into_iter().collect::<Vec<_>>();
    v.sort_unstable_by(|a, b| a.0.cmp(&b.0));
    v
}
