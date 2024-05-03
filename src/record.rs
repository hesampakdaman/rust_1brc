pub struct Record {
    min: f32,
    max: f32,
    sum: f32,
    count: usize,
}

impl Record {
    pub fn merge(&mut self, t: f32) {
        self.min = if t < self.min { t } else { self.min };
        self.max = if t > self.max { t } else { self.max };
        self.sum += t;
        self.count += 1;
    }
}

impl Default for Record {
    fn default() -> Self {
        Self {
            min: 0.0,
            max: 0.0,
            sum: 0.0,
            count: 0,
        }
    }
}

impl TryFrom<&str> for Record {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let t: f32 = value.split(';').collect::<Vec<_>>()[1]
            .parse()
            .map_err(|_| "Could not parse temprature")?;
        Ok(Record {
            min: t,
            max: t,
            sum: t,
            count: 1,
        })
    }
}

impl std::fmt::Display for Record {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}/{}/{}",
            self.min,
            self.max,
            self.sum / self.count as f32
        )
    }
}
