use crate::weather::{Report, Station};
use std::sync::mpsc::Receiver;

pub fn reduce(rx: Receiver<Report>) -> Vec<Station> {
    let mut hmap = Report::default();
    while let Ok(stats) = rx.recv() {
        merge_records(&mut hmap, stats);
    }
    to_sorted_vec(hmap)
}

fn merge_records(dst: &mut Report, src: Report) {
    for (city, new_record) in src.into_iter() {
        dst.entry(city)
            .and_modify(|existing_record: &mut Station| existing_record.merge(&new_record))
            .or_insert(new_record);
    }
}

fn to_sorted_vec(hmap: Report) -> Vec<Station> {
    let mut v = hmap.to_vec();
    v.sort_unstable();
    v
}
