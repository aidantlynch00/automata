use macroquad::time::get_frame_time;

pub struct GenerationTimer {
    gens_per_sec: f32,
    target: f32,
    sum: f32,
}

impl GenerationTimer {
    pub fn new(gens_per_sec: f32) -> GenerationTimer {
        GenerationTimer {
            gens_per_sec,
            target: 1.0 / gens_per_sec,
            sum: 0.0,
        }
    }

    pub fn generate(&mut self) -> bool {
        self.sum += get_frame_time();
        if self.sum >= self.target {
            self.sum -= self.target;
            true
        }
        else {
            false
        }
    }

    pub fn inc_rate(&mut self) {
        self.gens_per_sec += 1.0;
        self.target = 1.0 / self.gens_per_sec;
    }

    pub fn dec_rate(&mut self) {
        self.gens_per_sec -= 1.0;
        if self.gens_per_sec < 1.0 {
            self.gens_per_sec = 1.0;
        }

        self.target = 1.0 / self.gens_per_sec;
    }
}
