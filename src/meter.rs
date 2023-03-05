use std::time::Duration;

pub trait Meter {
    fn tick_duration(&self) -> Duration;
}

#[derive(Debug, Clone)]
pub struct Bpm {
    bpm: u32,
}

impl Meter for Bpm {
    fn tick_duration(&self) -> Duration {
        Duration::from_secs(60) / self.bpm
    }
}

impl Bpm {
    pub fn new(bpm: u32) -> Self {
        Bpm { bpm }
    }
}
