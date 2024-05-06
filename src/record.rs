#[derive(Debug, Default)]
pub struct Record {
    min: i32,
    max: i32,
    sum: i32,
    count: usize,
}

impl Record {
    pub fn merge(&mut self, other: Self) {
        self.min = std::cmp::min(self.min, other.min);
        self.max = std::cmp::max(self.max, other.max);
        self.sum += other.sum;
        self.count += other.count;
    }

    pub fn add(&mut self, t: i32) {
        self.min = t.min(self.min);
        self.max = t.max(self.max);
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
