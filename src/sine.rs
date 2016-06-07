use dsp::{Frame, Node, slice};
//use dsp::slice;

use base::{Amplitude, Frequency, Output, Phase, CHANNELS};
use math;

#[derive(Debug)]
pub struct Sine {
    pub frequency: Frequency,
    pub phase: Phase,
    pub amplitude: Amplitude
}

impl Node<[Output; CHANNELS]> for Sine {
    fn audio_requested(&mut self, buffer: &mut [[Output; CHANNELS]], sample_hz: f64) {
        slice::map_in_place(buffer, |_| {
            let wave: Amplitude = math::sine(self.phase);
            let sample = self.amplitude * wave;
            
            self.phase += self.frequency / sample_hz as Frequency;

            Frame::from_fn(|_| sample)
        });
    }
}