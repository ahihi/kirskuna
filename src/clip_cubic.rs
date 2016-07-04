use dsp::{Frame, Node, slice};

use base::{Output, CHANNELS};

#[derive(Debug)]
pub struct ClipCubic {
    pre_gain: Output,
    width: Output,
    post_gain: Output,
    mix: Output
}

impl ClipCubic {
    pub fn new(pre_gain: Output, width: Output, post_gain: Output, mix: Output) -> ClipCubic {
        ClipCubic { pre_gain: pre_gain, width: width, post_gain: post_gain, mix: mix }
    }
}

pub fn clip_cubic(w: Output, input: Output) -> Output {
    let magnitude = Output::abs(input);

    if magnitude > w {
        Output::signum(input) * (w - (w*w*w / 3.0))
    } else {
        input - input*input*input / 3.0
    }
}

impl Node<[Output; CHANNELS]> for ClipCubic {
    fn audio_requested(&mut self, buffer: &mut [[Output; CHANNELS]], _sample_hz: f64) {
        slice::map_in_place(buffer, |frame| frame.map(|sample| {
            clip_cubic(self.width, sample * self.pre_gain) * self.post_gain
        }));
    }
    
    fn dry(&self) -> Output {
        1.0 - self.mix
    }
    
    fn wet(&self) -> Output {
        self.mix
    }
}