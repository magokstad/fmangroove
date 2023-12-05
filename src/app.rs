
pub struct App {
    oscillators: Vec<Oscillator>,
    instruments: Vec<Box<dyn Instrument>>,
    instructions: Vec<Vec<Instruction>>,
    delay: u16,
}

impl App {
    pub fn new() -> Self {
        Self {
            oscillators: vec![Oscillator::default(), Oscillator::default()],
            delay: 125,
            notes: vec![],
        }
    }

    pub fn get_delay(&self) -> u16 {
        self.delay
    }

    pub fn set_delay(&mut self, delay: u16) {
        self.delay = delay;
    }

    pub fn get_bpm(&self) -> u16 {
        return 15000 / self.delay
    }

    pub fn set_bpm(&mut self, bpm: u16) {
        self.delay = 15000 / bpm
    }

    pub fn set_sample_rates(&mut self, sample_rate: f32) {
        for o in self.oscillators.iter_mut() {
            o.sample_rate = sample_rate;
        }
    }

    pub fn tick_oscillators(&mut self) -> f32 {
        // TODO: find cleaner way to handle amplitude
        self.oscillators.iter_mut().fold(0.0, |acc, x| {acc + x.tick()}) / 2f32
    }

    pub fn change_waveform(&mut self, i: usize, waveform: Waveform) {
        if let Some(o) = self.oscillators.get_mut(i) {
            o.waveform = waveform
        }
    }
}

pub trait Instrument {
    fn get_sound(&self) -> f32;

}

pub struct Instruction {
    note:
}

pub struct Oscillator {
    pub sample_rate: f32,
    pub waveform: Waveform,
    pub current_sample_index: f32,
    pub frequency_hz: f32,
    pub is_on: bool,
}

impl Oscillator {

    fn default() -> Self {
        Self {
            sample_rate: 0.0,
            waveform: Waveform::Sine,
            current_sample_index: 0.0,
            frequency_hz: 220.0,
            is_on: true
        }
    }

    fn advance_sample(&mut self) {
        self.current_sample_index = (self.current_sample_index + 1.0) % self.sample_rate;
    }

    pub fn set_waveform(&mut self, waveform: Waveform) {
        self.waveform = waveform;
    }

    fn calculate_sine_output_from_freq(&self, freq: f32) -> f32 {
        let two_pi = 2.0 * std::f32::consts::PI;
        (self.current_sample_index * freq * two_pi / self.sample_rate).sin()
    }

    fn is_multiple_of_freq_above_nyquist(&self, multiple: f32) -> bool {
        self.frequency_hz * multiple > self.sample_rate / 2.0
    }

    fn sine_wave(&mut self) -> f32 {
        self.calculate_sine_output_from_freq(self.frequency_hz)
    }

    fn generative_waveform(&mut self, harmonic_index_increment: i32, gain_exponent: f32) -> f32 {
        let mut output = 0.0;
        let mut i = 1;
        while !self.is_multiple_of_freq_above_nyquist(i as f32) {
            let gain = 1.0 / (i as f32).powf(gain_exponent);
            output += gain * self.calculate_sine_output_from_freq(self.frequency_hz * i as f32);
            i += harmonic_index_increment;
        }
        output
    }

    fn square_wave(&mut self) -> f32 {
        self.generative_waveform(2, 1.0)
    }

    fn saw_wave(&mut self) -> f32 {
        self.generative_waveform(1, 1.0)
    }

    fn triangle_wave(&mut self) -> f32 {
        self.generative_waveform(2, 2.0)
    }

    pub fn tick(&mut self) -> f32 {
        self.advance_sample();
        if !self.is_on {
            return 0f32
        }
        match self.waveform {
            Waveform::Sine => self.sine_wave(),
            Waveform::Square => self.square_wave(),
            Waveform::Saw => self.saw_wave(),
            Waveform::Triangle => self.triangle_wave(),
        }
    }
}

#[derive(Copy, Clone)]
pub enum Waveform {
    Sine,
    Square,
    Saw,
    Triangle,
}
