mod aggregate;
mod compute;
mod pre_processing;
mod weather;

use memmap2::MmapOptions;
use std::fs::File;
use std::path::PathBuf;
use std::sync::mpsc;
use std::sync::Arc;
use std::thread;

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() < 2 {
        eprintln!("Usage: {} </path/to/measurements.txt>", args[0]);
        std::process::exit(1);
    }
    let path = PathBuf::from(&args[1]);
    match run(path) {
        Ok(res) => print_results(&res),
        Err(RunErr::IO(e)) => eprintln!("{}", e),
    }
}

fn run(path: PathBuf) -> Result<Vec<weather::Station>, RunErr> {
    let file = File::open(path).unwrap();
    let mmap = Arc::new(unsafe { MmapOptions::new().map(&file).map_err(RunErr::IO)? });
    let (tx, rx) = mpsc::channel();
    pre_processing::Partition::try_from(&*mmap as &[u8])
        .map_err(RunErr::IO)?
        .chunks
        .into_iter()
        .for_each(|chunk| {
            let tx = tx.clone();
            let mmap = Arc::clone(&mmap);
            thread::spawn(move || compute::stats(&mmap[chunk], tx));
        });
    drop(tx);
    Ok(aggregate::reduce(rx))
}

#[derive(Debug)]
enum RunErr {
    IO(std::io::Error),
}

fn print_results(v: &[weather::Station]) {
    print!("{{");
    for (i, record) in v.iter().enumerate() {
        if i < v.len() - 1 {
            print!("{record}, ");
        } else {
            print!("{record}");
        }
    }
    println!("}}")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn integration() {
        let path = PathBuf::from("./data/measurements-test.txt");
        let actual = run(path).unwrap();
        let expected = vec![
            weather::Station::new("London", 85, 95, 180, 2),
            weather::Station::new("New York", 35, 150, 185, 2),
            weather::Station::new("Oslo", -100, 102, 2, 2),
            weather::Station::new("Paris", 130, 130, 130, 1),
            weather::Station::new("Stockholm", -5, 200, 210, 3),
        ];
        assert_eq!(actual, expected);
    }
}
