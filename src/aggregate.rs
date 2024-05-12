use crate::weather;
use std::sync::mpsc::Receiver;

pub fn reduce(rx: Receiver<weather::Report>) -> Vec<weather::Station> {
    let mut hmap = weather::Report::default();
    while let Ok(stats) = rx.recv() {
        merge_records(&mut hmap, stats);
    }
    to_sorted_vec(hmap)
}

fn merge_records(dst: &mut weather::Report, src: weather::Report) {
    for (city, new_record) in src.into_iter() {
        dst.entry(city)
            .and_modify(|existing_record: &mut weather::Station| existing_record.merge(&new_record))
            .or_insert(new_record);
    }
}

fn to_sorted_vec(hmap: weather::Report) -> Vec<weather::Station> {
    let mut v = hmap.to_vec();
    v.sort_unstable();
    v
}
