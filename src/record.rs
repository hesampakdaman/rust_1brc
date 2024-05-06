#[derive(Debug, Default)]
pub struct Record {
    min: i32,
    max: i32,
    sum: i32,
    count: usize,
}

impl Record {
    pub fn merge(&mut self, other: Self) {
        self.min = if other.min < self.min {
            other.min
        } else {
            self.min
        };
        self.max = if other.max > self.max {
            other.max
        } else {
            self.max
        };
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
