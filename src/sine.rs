use dsp::{Node, Settings};

use base::{Amplitude, Frequency, Output, Phase};
use math;

#[derive(Debug)]
pub struct Sine {
    pub frequency: Frequency,
    pub phase: Phase,
    pub amplitude: Amplitude
}

impl Node<Output> for Sine {
    fn audio_requested(&mut self, buffer: &mut [Output], settings: Settings) {
        for frame in buffer.chunks_mut(settings.channels as usize) {
            let wave: Amplitude = math::sine(self.phase);
            let sample = self.amplitude * wave;
            
            for channel in frame.iter_mut() {
                *channel = sample;
            }
            
            let sample_rate = settings.sample_hz as Frequency;
            self.phase += self.frequency / sample_rate;
        }        
    }
}