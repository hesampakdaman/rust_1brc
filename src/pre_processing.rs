use std::io::{self, BufRead, BufReader, Seek};

fn partition(path: &str, partitions: usize) -> Result<Vec<BytePart>, io::Error> {
    let file = std::fs::File::open(path)?;
    let bytes = file.metadata()?.len();
    let reader = io::BufReader::new(file);
    split(reader, partitions, bytes)
}

type Partition = Vec<BytePart>;

#[derive(Debug, PartialEq)]
struct BytePart {
    offset: u64,
    length: u64,
}

fn split<T: io::Read + io::Seek>(
    mut reader: BufReader<T>,
    partitions: usize,
    bytes: u64,
) -> Result<Partition, io::Error> {
    let mut partition = Vec::new();
    let mut offset: u64 = 0;
    let chunk_size = bytes / partitions as u64;
    let mut buf = Vec::with_capacity(chunk_size as usize);
    let mut rem_bytes = bytes as i64;
    while rem_bytes > 0 {
        let next_bytes = chunk_size.min(rem_bytes as u64);
        reader.seek(io::SeekFrom::Start(offset + next_bytes))?;
        let extra = reader.read_until(b'\n', &mut buf)? as u64;
        let length = next_bytes + extra;
        partition.push(BytePart { offset, length });
        offset += length;
        rem_bytes -= length as i64;
        buf.clear();
    }
    Ok(partition)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Cursor, Read, Seek};

    fn check(parts: Vec<BytePart>, expected: &str) {
        let contents = io::Cursor::new(expected.as_bytes());
        let mut actual = Vec::new();
        for p in &parts {
            let mut reader = io::BufReader::new(contents.clone());
            let mut buffer = vec![0; p.length as usize];
            reader.seek(io::SeekFrom::Start(p.offset)).unwrap();
            reader.read(&mut buffer).unwrap();
            actual.extend_from_slice(&buffer);
        }
        assert_eq!(String::from_utf8(actual).unwrap(), expected.to_string());
        assert_eq!(
            parts.iter().map(|p| p.length).sum::<u64>() as usize,
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
 Nam vitae gravida tortor.";
        let reader = BufReader::new(Cursor::new(data));
        let parts = split(reader, 4, data.len() as u64).unwrap();
        check(parts, data);
    }
}
