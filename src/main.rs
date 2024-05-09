mod aggregate;
mod compute;
mod pre_processing;
mod record;

use memmap2::MmapOptions;
use record::Record;
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

fn run(path: PathBuf) -> Result<Vec<WeatherReport>, RunErr> {
    let file = File::open(path).unwrap();
    let mmap = Arc::new(unsafe { MmapOptions::new().map(&file).map_err(RunErr::IO)? });
    let (tx, rx) = mpsc::channel();
    pre_processing::Partition::try_from(&*mmap as &[u8])
        .map_err(RunErr::IO)?
        .chunks
        .into_iter()
        .for_each(|chunk| {
            let mmap = Arc::clone(&mmap);
            let tx_clone = tx.clone();
            thread::spawn(move || compute::stats(&mmap[chunk], tx_clone));
        });
    drop(tx);
    Ok(aggregate::reduce(rx))
}

type WeatherReport = (String, Record);

#[derive(Debug)]
enum RunErr {
    IO(std::io::Error),
}

fn print_results(v: &[WeatherReport]) {
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn integration() {
        let path = PathBuf::from("./data/measurements-test.txt");
        let actual = run(path).unwrap();
        let expected = vec![
            (String::from("London"), Record::new(85, 95, 180, 2)),
            (String::from("New York"), Record::new(35, 150, 185, 2)),
            (String::from("Oslo"), Record::new(-100, 102, 2, 2)),
            (String::from("Paris"), Record::new(130, 130, 130, 1)),
            (String::from("Stockholm"), Record::new(-5, 200, 210, 3)),
        ];
        assert_eq!(actual, expected);
    }
}
