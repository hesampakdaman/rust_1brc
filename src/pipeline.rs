use crate::aggregate;
use crate::compute;
use crate::pre_processing;
use crate::weather;
use memmap2::Mmap;
use std::fs::File;
use std::sync::mpsc;
use std::sync::Arc;
use std::thread;

type Result = std::result::Result<Vec<weather::Station>, std::io::Error>;

pub fn run(file: &File) -> Result {
    let mmap = Arc::new(unsafe { Mmap::map(file)? });
    let (tx, rx) = mpsc::channel();
    pre_processing::Partition::try_from(&*mmap as &[u8])?
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

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::path::PathBuf;

    #[test]
    fn test_run() {
        let file = File::open(PathBuf::from("./data/measurements-test.txt"))
            .expect("Test file {path} not found");
        let actual = run(&file).unwrap();
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
