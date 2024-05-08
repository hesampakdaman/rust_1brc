#[derive(Debug, Default, PartialEq, Eq)]
pub struct Record {
    min: i32,
    max: i32,
    sum: i32,
    count: usize,
}

impl Record {
    pub fn new(min: i32, max: i32, sum: i32, count: usize) -> Self {
        Self {
            min,
            max,
            sum,
            count,
        }
    }

    pub fn merge(&mut self, other: &Self) {
        self.min = std::cmp::min(self.min, other.min);
        self.max = std::cmp::max(self.max, other.max);
        self.sum += other.sum;
        self.count += other.count;
    }

    pub fn add(&mut self, t: i32) {
        self.min = std::cmp::min(self.min, t);
        self.max = std::cmp::max(self.max, t);
        self.sum += t;
        self.count += 1;
    }
}

impl From<i32> for Record {
    fn from(value: i32) -> Self {
        Self {
            min: value,
            max: value,
            sum: value,
            count: 1,
        }
    }
}

impl std::fmt::Display for Record {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:.1}/{:.1}/{:.1}",
            self.min as f32 / 10.0,
            (self.sum as f32 / 10.0) / self.count as f32,
            self.max as f32 / 10.0,
        )
    }
}
