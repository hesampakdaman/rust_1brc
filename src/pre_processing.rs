use std::io;
use std::ops::Range;

pub struct Partition {
    pub chunks: Vec<Range<usize>>,
}

impl TryFrom<&[u8]> for Partition {
    type Error = io::Error;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        let n_threads = std::thread::available_parallelism()?.get();
        Ok(Splitter::new(bytes, n_threads).partition())
    }
}

struct Splitter<'a> {
    bytes: &'a [u8],
    chunk_size: usize,
    remaining_bytes: i64,
}

impl<'a> Splitter<'a> {
    fn new(bytes: &'a [u8], n: usize) -> Self {
        let remaining_bytes = bytes.len() as i64;
        let chunk_size = bytes.len() / n;
        Self {
            bytes,
            chunk_size,
            remaining_bytes,
        }
    }

    fn partition(mut self) -> Partition {
        let mut segments = Vec::new();
        let mut start = 0;
        while self.remaining_bytes > 0 {
            let end = self.get_chunk_end(start);
            segments.push(start..end);
            self.remaining_bytes -= (end - start) as i64;
            start = end;
        }
        Partition { chunks: segments }
    }

    fn get_chunk_end(&mut self, start: usize) -> usize {
        if self.remaining_bytes < self.chunk_size as i64 {
            return start + self.remaining_bytes as usize;
        }
        let size_to_newline = self.bytes[(start + self.chunk_size)..]
            .iter()
            .position(|&b| b == b'\n')
            .unwrap_or(0);
        start + self.chunk_size + size_to_newline + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Read, Seek};

    fn check(partition: Partition, bytes: &[u8], expected: Vec<&str>) {
        let contents = io::Cursor::new(bytes);
        for (i, chunk) in partition.chunks.iter().enumerate() {
            let mut reader = io::BufReader::new(contents.clone());
            let mut buffer = vec![0; chunk.len()];
            reader
                .seek(io::SeekFrom::Start(chunk.start as u64))
                .unwrap();
            reader.read(&mut buffer).unwrap();
            assert_eq!(std::str::from_utf8(&buffer).unwrap(), expected[i]);
        }
        let actual_bytes_read = partition.chunks.iter().map(|p| p.len()).sum::<usize>();
        assert_eq!(actual_bytes_read, bytes.len());
    }

    #[test]
    fn partition() {
        let data = "
Lorem ipsum dolor sit amet, consectetur adipiscing elit.
Nunc ac tempus sapien,
nec eleifend lacus. Curabitur vel imperdiet massa. Phasellus interdum
mattis eros quis iaculis. Nullam sed nulla vel dui pellentesque
bibendum quis et mauris. Integer vestibulum elementum metus,
in convallis arcu lectus."
            .trim_start();
        let expected = vec![
            "Lorem ipsum dolor sit amet, consectetur adipiscing elit.\nNunc ac tempus sapien,\n",
            "nec eleifend lacus. Curabitur vel imperdiet massa. Phasellus interdum\n",
            "mattis eros quis iaculis. Nullam sed nulla vel dui pellentesque\n",
            "bibendum quis et mauris. Integer vestibulum elementum metus,\n",
            "in convallis arcu lectus.",
        ];
        let bytes = data.as_bytes();
        let n_chunks = 5;
        let partition = Splitter::new(bytes, n_chunks).partition();
        check(partition, bytes, expected);
    }
}
