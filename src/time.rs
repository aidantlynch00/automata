use std::time::{Duration, SystemTime};

pub struct Ticker {
    ticks_per_secs: u32,
    target: Duration,
    last_tick: SystemTime,
}

impl Ticker {
    pub fn new(ticks_per_sec: u32) -> Ticker {
        Ticker {
            ticks_per_secs: ticks_per_sec,
            target: Duration::from_secs(1) / ticks_per_sec,
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
        self.ticks_per_secs += 1;
        self.target = Duration::from_secs(1) / self.ticks_per_secs;
    }

    pub fn dec_rate(&mut self) {
        if self.ticks_per_secs > 1 {
            self.ticks_per_secs -= 1;
            self.target = Duration::from_secs(1) / self.ticks_per_secs;
        }
    }
}
