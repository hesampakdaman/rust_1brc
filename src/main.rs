mod aggregate;
mod compute;
mod pre_processing;
mod record;

use memmap2::MmapOptions;
use std::fs::File;
use std::sync::mpsc;
use std::sync::Arc;
use std::thread;

fn main() {
    let file = File::open("./measurements.txt").unwrap();
    let mmap = Arc::new(unsafe { MmapOptions::new().map(&file).unwrap() });
    let (tx, rx) = mpsc::channel();
    pre_processing::Partition::try_from(file)
        .unwrap()
        .chunks
        .into_iter()
        .for_each(|chunk| {
            let mmap_clone = Arc::clone(&mmap);
            let tx_clone = tx.clone();
            thread::spawn(move || compute::stats(mmap_clone, chunk, tx_clone));
        });
    drop(tx);
    print_results(&aggregate::reduce(rx));
}

fn print_results(v: &[(String, record::Record)]) {
    print!("{{");
    for (i, (name, r)) in v.iter().enumerate() {
        if i < v.len() - 1 {
            print!("{name}: {r}, ");
        } else {
            print!("{name}: {r}");
        }
    }
    println!("}}")
}
