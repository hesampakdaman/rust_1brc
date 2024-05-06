use memmap2::Mmap;
use std::io;

pub struct Partition {
    pub chunks: Vec<Chunk>,
}

impl TryFrom<&Mmap> for Partition {
    type Error = io::Error;

    fn try_from(mmap: &Mmap) -> Result<Self, Self::Error> {
        let bytes = mmap.len();
        let n_threads = std::thread::available_parallelism()?.get();
        let splitter = Splitter::new(mmap, (bytes / n_threads) as u64);
        splitter.partition()
    }
}

#[derive(Debug, PartialEq)]
pub struct Chunk {
    pub offset: u64,
    pub size: u64,
}

struct Splitter<'a> {
    bytes: &'a [u8],
    chunk_size: u64,
    remaining_bytes: i64,
}

impl<'a> Splitter<'a> {
    fn new(bytes: &'a [u8], chunk_size: u64) -> Self {
        let remaining_bytes = bytes.len() as i64;
        Self {
            bytes,
            chunk_size,
            remaining_bytes,
        }
    }

    fn partition(mut self) -> Result<Partition, io::Error> {
        let mut segments = Vec::new();
        let mut offset: u64 = 0;
        while self.remaining_bytes > 0 {
            let size = self.next_chunk_size(offset)?;
            segments.push(Chunk { offset, size });
            offset += size;
            self.remaining_bytes -= size as i64;
        }
        Ok(Partition { chunks: segments })
    }

    fn next_chunk_size(&mut self, offset: u64) -> Result<u64, io::Error> {
        let estimated_end = std::cmp::min(self.chunk_size, self.remaining_bytes as u64);
        let bytes = &self.bytes[(offset+estimated_end) as usize..];
        let mut i = 0;
        while  i < bytes.len() && bytes[i] != b'\n' {
            i += 1;
        }
        Ok(estimated_end + i as u64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Read, Seek};

    fn check(partition: Partition, expected: &str) {
        let contents = io::Cursor::new(expected.as_bytes());
        let mut actual = Vec::new();
        for chunk in &partition.chunks {
            let mut reader = io::BufReader::new(contents.clone());
            let mut buffer = vec![0; chunk.size as usize];
            reader.seek(io::SeekFrom::Start(chunk.offset)).unwrap();
            reader.read(&mut buffer).unwrap();
            actual.extend_from_slice(&buffer);
        }
        assert_eq!(String::from_utf8(actual).unwrap(), expected.to_string());
        assert_eq!(
            partition.chunks.iter().map(|p| p.size).sum::<u64>() as usize,
            expected.as_bytes().len()
        );
    }

    #[test]
    fn test() {
        let data = "Lorem ipsum dolor sit amet, consectetur adipiscing elit.
 Quisque eget justo non magna ultricies cursus non posuere velit.
 Nunc blandit, elit in tincidunt luctus, est neque cursus libero, eleifend posuere magna nisl vel dui.
 Aliquam erat volutpat.
 Lorem ipsum dolor sit amet, consectetur adipiscing elit.
 Nam laoreet enim in condimentum semper.
 Ut vel leo faucibus, sodales augue volutpat, lobortis urna.
 Nam vitae gravida tortor.
";
        let bytes = data.as_bytes();
        let partition = Splitter::new(bytes, 4)
            .partition()
            .unwrap();
        check(partition, data);
    }
}
