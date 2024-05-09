use crate::record::Record;
use std::collections::HashMap;
use std::sync::mpsc::Sender;

pub fn stats(bytes: &[u8], tx: Sender<HashMap<String, Record>>) {
    let hmap = calculate(bytes);
    tx.send(hmap).unwrap();
}

fn calculate(bytes: &[u8]) -> HashMap<String, Record> {
    let mut map: HashMap<String, Record> = HashMap::with_capacity(10_000);
    for line in bytes.split(|&b| b == b'\n') {
        if !line.is_empty() {
            let mut splitted = line.split(|&b| b == b';');
            let city = unsafe { std::str::from_utf8_unchecked(splitted.next().unwrap()) };
            let num = parse_float(splitted.next().unwrap());
            if let Some(rec) = map.get_mut(city) {
                rec.add(num);
            } else {
                map.insert(city.to_string(), Record::from(num));
            }
        }
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

    fn check(data: &str, expected: HashMap<String, Record>) {
        let actual = calculate(data.as_bytes());
        assert_eq!(actual, expected);
    }

    #[test]
    fn compute() {
        let input = "Stockholm;1.5
New York;2.0
Oslo;0.0
Stockholm;11.5
Oslo;10.2";
        let expected = HashMap::from([
            ("Stockholm".to_string(), Record::new(15, 115, 130, 2)),
            ("New York".to_string(), Record::new(20, 20, 20, 1)),
            ("Oslo".to_string(), Record::new(0, 102, 102, 2)),
        ]);
        check(input, expected);
    }
}
