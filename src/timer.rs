use std::time::{Instant, Duration};

pub struct Timer {
    prev_time: Instant,
    crnt_time: Instant
}

impl Timer {
    pub fn new() -> Self {
        Timer {
            prev_time: std::time::Instant::now(),
            crnt_time: std::time::Instant::now()
        }
    }

    pub fn update(&mut self) {
        self.crnt_time = std::time::Instant::now();
    }

    pub fn elapsed(&self) -> Duration {
        self.crnt_time - self.prev_time
    }

    pub fn reset(&mut self) {
        self.prev_time = self.crnt_time
    }
}
