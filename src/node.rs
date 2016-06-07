use dsp::{Node};

use base::{Output, CHANNELS};
use delay;
use sine;

#[derive(Debug)]
pub enum DspNode {
    Blank,
    Sine(sine::Sine),
    Delay(delay::Delay)
}

impl DspNode {
}

impl Node<[Output; CHANNELS]> for DspNode {
    fn audio_requested(&mut self, buffer: &mut [[Output; CHANNELS]], sample_hz: f64) {
        match *self {
            DspNode::Blank => (),
            DspNode::Sine(ref mut node) => node.audio_requested(buffer, sample_hz),
            DspNode::Delay(ref mut node) => node.audio_requested(buffer, sample_hz)
        }        
    }
    
    fn dry(&self) -> Output {
        match *self {
            DspNode::Blank => 0.0,
            DspNode::Sine(ref node) => node.dry(),
            DspNode::Delay(ref node) => node.dry(),
        }
    }
    
    fn wet(&self) -> Output {
        match *self {
            DspNode::Blank => 1.0,
            DspNode::Sine(ref node) => node.wet(),
            DspNode::Delay(ref node) => node.wet(),
        }
    }
}