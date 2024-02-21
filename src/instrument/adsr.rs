enum AdsrState {
    Pressed,
    Released,
    Off,
}

pub struct Adsr {
    state: AdsrState,
    sample_rate: f32,
    frame: u128,
    attack: f32,
    decay: f32,
    sustain: f32,
    release: f32,
}

impl Adsr {
    pub fn new(a: f32, d: f32, s: f32, r: f32) -> Self {
        Self {
            state: AdsrState::Off,
            sample_rate: 0.0,
            // Used for frames since pressed, AND frames since released
            frame: 0,
            attack: a.max(0.0),
            decay: d.max(0.0),
            sustain: s.clamp(0.0, 1.0),
            release: r.max(0.0),
        }
    }

    pub fn set_sample_rate(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate
    }

    pub fn press(&mut self) {
        self.state = AdsrState::Pressed;
        self.frame = 0;
    }

    pub fn release(&mut self) {
        self.state = AdsrState::Released;
        self.frame = 0;
    }

    pub fn stop(&mut self) {
        self.state = AdsrState::Off;
        self.frame = 0;
    }

    pub fn tick(&mut self) -> f32 {
        let cur_frame = self.frame as f32;
        if self.frame != u128::MAX {
            self.frame += 1;
        }
        match self.state {
            AdsrState::Off => 0.0,
            AdsrState::Pressed => {
                let attack_frames = self.attack * self.sample_rate;
                let decay_frames = self.decay * self.sample_rate;
                if cur_frame < attack_frames {
                    // Attack
                    cur_frame / attack_frames
                } else if cur_frame < attack_frames + decay_frames {
                    // Decay
                    (1.0 - ((cur_frame - attack_frames) / decay_frames)).clamp(self.sustain, 1.0)
                } else {
                    // Sustain
                    self.sustain
                }
            }
            AdsrState::Released => {
                // Release
                let release_frames = self.release * self.sample_rate;
                (1.0 - (cur_frame / release_frames)) * self.sustain
            }
        }
        .clamp(0.0, 1.0)
    }
}
