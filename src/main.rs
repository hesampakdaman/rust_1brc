mod compute;
mod pre_processing;
mod record;
mod aggregate;

use std::fs::File;
use std::sync::mpsc;
use std::thread;

fn main() {
    let file = File::open("./measurements.txt").unwrap();
    let (tx, rx) = mpsc::channel();
    pre_processing::Partition::try_from(file)
        .unwrap()
        .chunks
        .into_iter()
        .for_each(|chunk| {
            let tx_clone = tx.clone();
            thread::spawn(move || compute::stats(chunk, tx_clone));
        });
    drop(tx);
    aggregate::reduce(rx);
}
