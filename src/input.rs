use dsp::{Frame, Node, slice};

use base::{Output, CHANNELS};
use midi::{MidiDestination};

#[derive(Debug)]
pub struct Input {
    buf: Vec<[Output; CHANNELS]>
}

impl Input {
    pub fn new(buffer_size: usize) -> Input {
        let mut buf = Vec::new();
        for _ in 0 .. buffer_size {
            buf.push(Frame::equilibrium());
        }
        
        Input { buf: buf }
    }
    
    pub fn fill<I: Iterator<Item=[Output; CHANNELS]>>(&mut self, frames: I) {
        for (dst, src) in self.buf.iter_mut().zip(frames) {
            *dst = src;
        }        
    }
}

impl Node<[Output; CHANNELS]> for Input {
    fn audio_requested(&mut self, buffer: &mut [[Output; CHANNELS]], _sample_hz: f64) {
        slice::zip_map_in_place(buffer, &self.buf, |_, src| src);
    }
}

impl MidiDestination for Input {}
