use dsp::{Node};

use base::{Output, CHANNELS};
use sine;

#[derive(Debug)]
pub enum DspNode {
    Blank,
    Sine(sine::Sine)
}

impl DspNode {
}

impl Node<[Output; CHANNELS]> for DspNode {
    fn audio_requested(&mut self, buffer: &mut [[Output; CHANNELS]], sample_hz: f64) {
        match *self {
            DspNode::Blank => (),
            DspNode::Sine(ref mut node) => node.audio_requested(buffer, sample_hz)
        }        
    }
}