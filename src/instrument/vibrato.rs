
pub struct Vibrato {
    rate: f32,
    depth: f32,
    sample_rate: f32,
    current_phase: f32,
    is_on: bool,
}

impl Vibrato {
    pub fn new(rate: f32, depth: f32) -> Self {
        Self {
            rate,
            depth,
            sample_rate: 0.0,
            current_phase: 0.0,
            is_on: false
        }
    }

    pub fn set_state(&mut self, is_on: bool) {
        self.is_on = is_on
    }

    pub fn set_sample_rate(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate;
    }

    pub fn set_rate_and_depth(&mut self, rate: f32, depth: f32) {
        self.rate = rate;
        self.depth = depth;
    }

    pub fn tick(&mut self) -> f32 {
        if !self.is_on {
            return 1.0
        }

        let modulation = (( self.rate * 2.0 * std::f32::consts::PI * self.current_phase) / self.sample_rate) .sin();
        self.current_phase = (self.current_phase + 1.0 ) % self.sample_rate;
        self.depth * modulation + 1.0
    }
}
