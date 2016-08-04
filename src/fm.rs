use dsp::{Frame, Node, slice};

use base::{CHANNELS, TAU};
use math;
use sine::Sine;

#[derive(Debug)]
pub struct Operator {
    base_frequency: f64,
    sine: Sine
}

impl Operator {
    pub fn new(base_frequency: f64, amplitude: f32) -> Operator {
        Operator {
            base_frequency: base_frequency,
            sine: Sine {
                frequency: base_frequency,
                phase: 0.0,
                amplitude: amplitude
            }
        }
    }
    
    pub fn value(&self) -> f32 {
        self.sine.value()
    }
    
    pub fn step(&mut self, sample_hz: f64) {
        self.sine.step(sample_hz);
        // TODO: amp env
    }
}

#[derive(Debug)]
pub struct FmSynth {
    pub carrier: Operator,
    pub modulator: Operator
}

impl Node<[f32; CHANNELS]> for FmSynth {
    fn audio_requested(&mut self, buffer: &mut [[f32; CHANNELS]], sample_hz: f64) {
        slice::map_in_place(buffer, |_| {
            let output = self.carrier.value();
            self.carrier.sine.frequency = self.carrier.base_frequency + self.modulator.value() as f64;
            self.carrier.step(sample_hz);
            self.modulator.step(sample_hz);
            
            Frame::from_fn(|_| output)
        });
    }
}