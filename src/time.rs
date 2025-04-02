use crate::EMA;
use std::collections::VecDeque;
use std::time::{Duration, Instant};

pub struct Ticker {
    start: Instant,
    interval: Duration,
    last: Instant,
    dt: Duration,
    acc: Duration,
}

impl Default for Ticker {
    fn default() -> Self {
        Self::new(Duration::default())
    }
}

impl Ticker {
    pub fn new(interval: Duration) -> Self {
        Self {
            start: Instant::now(),
            interval,
            last: Instant::now(),
            dt: Duration::default(),
            acc: Duration::default(),
        }
    }
    pub fn tick(&mut self) -> usize {
        let now = Instant::now();
        self.dt = now - self.last;
        self.last = now;

        let mut tick_count = 0;
        if !self.interval.is_zero() {
            self.acc += self.dt;
            while self.acc >= self.interval {
                self.acc -= self.interval;
                tick_count += 1;
            }
        }

        tick_count
    }

    pub fn dt(&self) -> Duration {
        self.dt
    }

    pub fn elapsed(&self) -> Duration {
        self.last - self.start
    }
}

pub struct FrameCounter {
    #[allow(dead_code)]
    start: Instant,
    timestamps: VecDeque<Instant>,
    ticker: Ticker,
    averager: EMA,
    user_ticker: Ticker,
}

impl Default for FrameCounter {
    fn default() -> Self {
        Self::new(Duration::default())
    }
}

impl FrameCounter {
    pub fn new(user_interval: Duration) -> Self {
        Self {
            start: Instant::now(),
            timestamps: VecDeque::default(),
            ticker: Ticker::new(Duration::from_millis(500)),
            averager: EMA::new(1.0),
            user_ticker: Ticker::new(user_interval),
        }
    }

    pub fn update(&mut self) -> usize {
        self.timestamps.push_back(Instant::now());

        while self.timestamps.len() > 2
            && *self.timestamps.back().unwrap() - *self.timestamps.front().unwrap()
                > Duration::from_secs(1)
        {
            self.timestamps.pop_front();
        }

        self.averager.update(self.timestamps.len() as f64);
        if self.ticker.tick() > 0 {
            self.averager.alpha = 2.0 / (1.0 + self.timestamps.len() as f64);
        }

        self.user_ticker.tick()
    }

    pub fn fps(&self) -> f64 {
        self.averager.value()
    }

    pub fn dt(&self) -> Duration {
        if self.timestamps.len() < 2 {
            Duration::default()
        } else {
            *self.timestamps.back().unwrap() - *self.timestamps.front().unwrap()
        }
    }
}
