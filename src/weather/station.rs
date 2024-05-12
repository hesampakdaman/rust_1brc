#[derive(Debug, Default, PartialEq, Eq)]
pub struct Station {
    pub name: String,
    min: i32,
    max: i32,
    sum: i32,
    count: usize,
}

impl Station {
    pub fn new(name: &str, min: i32, max: i32, sum: i32, count: usize) -> Self {
        Self {
            name: name.to_string(),
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

    fn average(&self) -> f32 {
        (self.sum as f32 / 10.0) / self.count as f32
    }

    fn min(&self) -> f32 {
        self.min as f32 / 10.0
    }

    fn max(&self) -> f32 {
        self.max as f32 / 10.0
    }
}

impl PartialOrd for Station {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Station {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.name.cmp(&other.name)
    }
}

impl From<(&str, i32)> for Station {
    fn from(value: (&str, i32)) -> Self {
        Self::new(value.0, value.1, value.1, value.1, 1)
    }
}

impl std::fmt::Display for Station {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: {:.1}/{:.1}/{:.1}",
            self.name,
            self.min(),
            self.average(),
            self.max(),
        )
    }
}
