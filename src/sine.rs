use dsp::{Frame, Node, slice};
//use dsp::slice;

use base::{Amplitude, Frequency, Output, Phase, CHANNELS, TAU};
use math;

#[derive(Debug)]
pub struct Sine {
    pub frequency: Frequency,
    pub phase: Phase,
    pub amplitude: Amplitude
}

impl Sine {
    pub fn value(&self) -> f32 {
        let sine: f32 = math::sine(self.phase);
        self.amplitude * sine
    }
    
    pub fn step(&mut self, sample_hz: f64) {
        self.phase = math::step_phase(self.frequency * TAU / sample_hz, self.phase);
    }
}

impl Node<[Output; CHANNELS]> for Sine {
    fn audio_requested(&mut self, buffer: &mut [[Output; CHANNELS]], sample_hz: f64) {
        slice::map_in_place(buffer, |_| {
            let sample = self.value();
            
            self.step(sample_hz);

            Frame::from_fn(|_| sample)
        });
    }
}