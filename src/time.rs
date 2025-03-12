use std::time::{Duration, SystemTime};

pub struct GenerationTimer {
    gens_per_sec: u32,
    target: Duration,
    last_tick: SystemTime,
}

impl GenerationTimer {
    pub fn new(gens_per_sec: u32) -> GenerationTimer {
        GenerationTimer {
            gens_per_sec,
            target: Duration::from_secs(1) / gens_per_sec,
            last_tick: SystemTime::UNIX_EPOCH,
        }
    }

    pub fn tick(&mut self) -> bool {
        let now = SystemTime::now();
        let time_since_tick = now.duration_since(self.last_tick).unwrap();

        if time_since_tick > self.target {
            self.last_tick = now;
            true
        }
        else {
            false
        }
    }

    pub fn inc_rate(&mut self) {
        self.gens_per_sec += 1;
        self.target = Duration::from_secs(1) / self.gens_per_sec;
    }

    pub fn dec_rate(&mut self) {
        if self.gens_per_sec > 1 {
            self.gens_per_sec -= 1;
            self.target = Duration::from_secs(1) / self.gens_per_sec;
        }
    }
}
