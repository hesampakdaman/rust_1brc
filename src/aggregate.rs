use crate::compute::CityKey;
use crate::record::Record;
use fxhash::FxHashMap;
use std::sync::mpsc::Receiver;

pub fn reduce(rx: Receiver<FxHashMap<CityKey, Record>>) -> Vec<Record> {
    let mut hmap = FxHashMap::default();
    while let Ok(stats) = rx.recv() {
        merge_records(&mut hmap, stats);
    }
    to_sorted_vec(hmap)
}

fn merge_records(dst: &mut FxHashMap<CityKey, Record>, src: FxHashMap<CityKey, Record>) {
    for (city, new_record) in src {
        dst.entry(city)
            .and_modify(|existing_record: &mut Record| existing_record.merge(&new_record))
            .or_insert(new_record);
    }
}

fn to_sorted_vec(hmap: FxHashMap<CityKey, Record>) -> Vec<Record> {
    let mut v = hmap.into_values().collect::<Vec<_>>();
    v.sort_unstable_by(|a, b| a.name.cmp(&b.name));
    v
}
