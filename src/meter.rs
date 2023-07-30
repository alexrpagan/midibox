use std::ops::Div;
use std::time::Duration;
use log::{debug, log};

pub trait Meter {
    fn tick_duration(&mut self) -> Duration;
}

#[derive(Debug, Clone)]
pub struct Bpm {
    bpm: u32,
}

impl Meter for Bpm {
    fn tick_duration(&mut self) -> Duration {
        Duration::from_secs(60) / self.bpm
    }
}

impl Bpm {
    pub fn new(bpm: u32) -> Self {
        Bpm { bpm }
    }
}


pub struct Oscillate {
    min_bpm: u32,
    max_bpm: u32,
    current_bpm: f64,
    step: f64,
    accel: bool,
}

impl Oscillate {
    pub fn new(min_bpm: u32, max_bpm: u32, step: f64) -> Self {
        Oscillate {
            min_bpm, max_bpm, step, accel: true, current_bpm: min_bpm as f64
        }
    }
}

impl Meter for Oscillate {
    fn tick_duration(&mut self) -> Duration {
        let curr_time = Duration::from_secs(60) / self.current_bpm as u32;
        if self.current_bpm >= self.max_bpm as f64 {
            self.current_bpm = self.max_bpm as f64;
            self.accel = false;
        } else if self.current_bpm <= self.min_bpm as f64 {
            self.current_bpm = self.min_bpm as f64;
            self.accel = true;
        }
        if self.accel {
            self.current_bpm += self.step;
        } else {
            self.current_bpm -= self.step;
        }
        debug!("current: {}, current_time: {}", self.current_bpm, curr_time.as_millis());
        curr_time
    }
}