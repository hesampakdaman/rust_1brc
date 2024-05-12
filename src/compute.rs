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
        bytes = if end_idx < bytes.len() {
            &bytes[end_idx + 1..]
        } else {
            &[]
        };
    }
    map
}

fn parse_float(bytes: &[u8]) -> i32 {
    let mut result = 0;
    let mut is_negative = false;
    for &b in bytes {
        match b {
            b'0'..=b'9' => {
                let digit = (b - b'0') as i32;
                result = result * 10 + digit;
            }
            b'-' => is_negative = true,
            _ => {}
        }
    }
    if is_negative {
        result *= -1;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(input: &str, expected: Vec<weather::Station>) {
        let map = calculate(input.as_bytes());
        let mut actual: Vec<weather::Station> = map.to_vec();
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
