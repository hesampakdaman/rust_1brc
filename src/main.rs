mod compute;
mod pre_processing;
mod record;
mod aggregate;

use std::fs::File;
use std::sync::mpsc;
use std::thread;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let file = File::open(args[1].clone()).expect("...");
    let (tx, rx) = mpsc::channel();
    pre_processing::Partition::try_from(file)
        .unwrap()
        .chunks
        .into_iter()
        .for_each(|chunk| {
            let tx_clone = tx.clone();
            thread::spawn(move || compute::stats(chunk, tx_clone));
        });
    aggregate::reduce(rx);
}
