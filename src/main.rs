mod aggregate;
mod compute;
mod pipeline;
mod pre_processing;
mod weather;

use std::fs::File;
use std::path::PathBuf;

fn main() -> Result<(), std::io::Error> {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() < 2 {
        eprintln!("Usage: {} </path/to/measurements.txt>", &args[0]);
        std::process::exit(1);
    }
    let file = File::open(PathBuf::from(&args[1]))?;
    let res = pipeline::run(&file)?;
    print_formatted(&res);
    Ok(())
}

fn print_formatted(stations: &[weather::Station]) {
    let s = stations
        .iter()
        .map(|st| st.to_string())
        .collect::<Vec<String>>()
        .join(", ");
    println!("{{{s}}}");
}
