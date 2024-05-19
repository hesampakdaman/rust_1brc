use crate::weather;
use memchr::memchr;
use std::sync::mpsc::Sender;

pub fn stats(bytes: &[u8], tx: Sender<weather::Report>) {
    let hmap = calculate(bytes);
    tx.send(hmap).unwrap();
}

fn calculate(mut bytes: &[u8]) -> weather::Report {
    let mut map = weather::Report::default();
    while let Some(sep_idx) = memchr(b';', bytes) {
        let end_idx = memchr(b'\n', bytes).unwrap_or(bytes.len());
        let key = weather::Key::new(&bytes[..sep_idx]);
        let num = parse_float(&bytes[sep_idx + 1..end_idx]);
        if let Some(rec) = map.get_mut(&key) {
            rec.add(num);
        } else {
            let name = unsafe { std::str::from_utf8_unchecked(&bytes[..sep_idx]) };
            map.insert(key, weather::Station::from((name, num)));
        }
        bytes = bytes.get(end_idx+1..).unwrap_or(&[]);
    }
    map
}

fn parse_float(bytes: &[u8]) -> i32 {
    // if the first byte has a minus sign we will skip it in the
    // iterator because neg would be 1 in that case
    let neg = (bytes[0] == b'-') as usize;
    let sgn = 1 - 2 * neg as i32;
    let res = bytes.iter().skip(neg).fold(0, |acc, &byte| {
        let is_digit = byte.is_ascii_digit() as i32;
        let digit = (byte as i32).wrapping_sub(b'0' as i32);
        acc * (10 * is_digit + (1 - is_digit)) + digit * is_digit
    });
    sgn * res
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(input: &str, expected: Vec<weather::Station>) {
        let map = calculate(input.as_bytes());
        let mut actual: Vec<weather::Station> = map.into_vec();
        actual.sort_unstable();
        assert_eq!(actual, expected);
    }

    #[test]
    fn compute() {
        let input = "
Stockholm;1.5
New York;2.0
Oslo;0.0
Stockholm;11.5
Oslo;10.2"
            .trim();
        let expected = vec![
            weather::Station::new("New York", 20, 20, 20, 1),
            weather::Station::new("Oslo", 0, 102, 102, 2),
            weather::Station::new("Stockholm", 15, 115, 130, 2),
        ];
        check(input, expected);
    }
}
