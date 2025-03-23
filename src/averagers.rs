use std::collections::VecDeque;

pub struct CMA {
    value: f64,
    sample_count: usize,
}

impl CMA {
    pub fn new() -> Self {
        Self {
            value: 0.0,
            sample_count: 0,
        }
    }

    pub fn update(&mut self, value: f64) {
        self.sample_count += 1;
        self.value += (value - self.value) / self.sample_count as f64;
    }

    pub fn value(&self) -> f64 {
        self.value
    }
}

pub struct EMA {
    pub alpha: f64,
    value: f64,
}

impl EMA {
    pub fn new(alpha: f64) -> Self {
        Self { alpha, value: 0.0 }
    }

    pub fn update(&mut self, value: f64) {
        self.value = self.alpha * value + (1.0 - self.alpha) * self.value;
    }

    pub fn value(&self) -> f64 {
        self.value
    }
}

pub struct SMA {
    pub sample_count: usize,
    samples: VecDeque<f64>,
    value: f64,
}

impl SMA {
    pub fn new(sample_count: usize) -> Self {
        Self {
            sample_count,
            samples: Default::default(),
            value: 0.0,
        }
    }

    pub fn update(&mut self, value: f64) {
        self.samples.push_back(value);
        if self.samples.len() <= self.sample_count {
            self.value += (value - self.value) / self.samples.len() as f64;
        } else {
            self.value +=
                (1.0 / self.sample_count as f64) * (value - self.samples.front().unwrap());
            self.samples.pop_back();
        }
    }

    pub fn value(&self) -> f64 {
        self.value
    }
}
