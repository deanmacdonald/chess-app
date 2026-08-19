use std::time::{Instant, Duration};

pub struct TimeManager {
    start: Instant,
    soft_limit: Duration,
    hard_limit: Duration,
}

impl TimeManager {
    pub fn new() -> Self {
        Self {
            start: Instant::now(),
            soft_limit: Duration::from_millis(10_000),
            hard_limit: Duration::from_millis(12_000),
        }
    }

    pub fn start(&mut self) {
        self.start = Instant::now();
    }

    pub fn set_movetime(&mut self, ms: u64) {
        self.soft_limit = Duration::from_millis(ms);
        self.hard_limit = Duration::from_millis(ms + ms / 5);
    }

    pub fn set_clock(&mut self, time_ms: u64, inc_ms: u64) {
        let alloc = time_ms / 30 + inc_ms;
        self.soft_limit = Duration::from_millis(alloc);
        self.hard_limit = Duration::from_millis(alloc + alloc / 5);
    }

    pub fn should_stop(&self) -> bool {
        self.start.elapsed() >= self.soft_limit
    }

    pub fn must_stop(&self) -> bool {
        self.start.elapsed() >= self.hard_limit
    }
}
