use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read, Seek};
use std::sync::mpsc::Sender;

use crate::pre_processing::Chunk;
use crate::record::Record;

type Statistics = HashMap<String, Record>;

pub fn stats(chunk: Chunk, tx: Sender<Statistics>) {
    let f = std::fs::File::open("./measurements.txt").unwrap();
    let reader = std::io::BufReader::new(f);
    let s = chunk.offset;
    for line in ChunkReader::new(reader, chunk) {
        if s == 0 {
        println!("{}", line);
        }

    }
}

struct ChunkReader<T: Read + Seek> {
    reader: BufReader<T>,
    size_bytes: usize,
}

impl<T: Read + Seek> ChunkReader<T> {
    fn new(mut reader: BufReader<T>, chunk: Chunk) -> Self {
        reader.seek(std::io::SeekFrom::Start(chunk.offset)).unwrap();
        let size_bytes = chunk.size as usize;
        Self { reader, size_bytes }
    }
}

impl<T: Read + Seek> Iterator for ChunkReader<T> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.size_bytes <= 0 {
            return None;
        }
        let mut buf = String::new();
        self.size_bytes -= self.reader.read_line(&mut buf).unwrap();
        Some(buf)
    }
}
