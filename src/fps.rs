#[derive(Clone)]
pub struct FpsModel {
    pub frames: u64,
    pub time: f64,
}

impl FpsModel {
    pub fn new() -> FpsModel {
        FpsModel {
            frames: 0,
            time: 0.0,
        }
    }

    pub fn log_frame(&mut self, frame_time: f64) {
        self.frames += 1;
        self.time += frame_time;
    }

    pub fn average_frame_time(&self) -> f64 {
        match self.frames {
            0 => 0.0,
            f => self.time / (f as f64)
        }

    }

    pub fn average_fps(&self) -> f64 {
        match self.time {
            0.0 => 0.0,
            t => (self.frames as f64) / t * 1000.0,
        }
    }

    pub fn reset(&mut self) {
        self.frames = 0;
        self.time = 0.0;
    }
}