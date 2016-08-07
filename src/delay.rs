use dsp::{Frame, Node, slice};

use base::{Output, CHANNELS};
use midi::{MidiDestination};

#[derive(Debug)]
pub struct Delay {
    buf: Vec<[Output; CHANNELS]>,
    index: usize
}

impl Delay {
    pub fn new(buffer_size: usize) -> Delay {
        let mut buf = Vec::new();
        for _ in 0 .. buffer_size {
            buf.push(Frame::equilibrium());
        }
        
        Delay { buf: buf, index: 0 }
    }
}

impl Node<[Output; CHANNELS]> for Delay {
    fn audio_requested(&mut self, buffer: &mut [[Output; CHANNELS]], _sample_hz: f64) {
        slice::map_in_place(buffer, |in_frame| {
            
            let out_frame = self.buf[self.index];
            self.buf[self.index] = in_frame;
            self.index = (self.index + 1) % self.buf.len();
                        
            out_frame
        });
    }
    
    fn dry(&self) -> Output {
        0.66
    }
    
    fn wet(&self) -> Output {
        0.33
    }
}

impl MidiDestination for Delay {}
