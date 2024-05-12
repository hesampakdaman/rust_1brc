use memchr::memchr;
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
}

impl<'a> Splitter<'a> {
    fn new(bytes: &'a [u8], n: usize) -> Self {
        let chunk_size = bytes.len() / n;
        Self { bytes, chunk_size }
    }

    fn partition(mut self) -> Partition {
        let mut chunks = Vec::new();
        let mut offset = 0;
        while self.bytes.len() > 0 {
            let end = self.get_chunk_end();
            chunks.push(offset..offset + end);
            self.bytes = &self.bytes[end..];
            offset += end;
        }
        Partition { chunks }
    }

    fn get_chunk_end(&mut self) -> usize {
        if self.bytes.len() < self.chunk_size {
            return self.bytes.len();
        }
        let chunk_end = &self.bytes[self.chunk_size..];
        let idx_to_newline = self.handle_chunk_ending_in_the_middle_of_sentence(chunk_end);
        self.chunk_size + idx_to_newline + 1
    }

    #[inline]
    fn handle_chunk_ending_in_the_middle_of_sentence(&self, bytes: &[u8]) -> usize {
        memchr(b'\n', bytes).unwrap_or_default()
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
