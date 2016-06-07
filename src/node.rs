use dsp::{Node, Settings};

use base::{Output};
use sine;

#[derive(Debug)]
pub enum DspNode {
    Blank,
    Sine(sine::Sine)
}

impl DspNode {
}

impl Node<Output> for DspNode {
    fn audio_requested(&mut self, buffer: &mut [Output], settings: Settings) {
        match *self {
            DspNode::Blank => (),
            DspNode::Sine(ref mut node) => node.audio_requested(buffer, settings)
        }        
    }
}