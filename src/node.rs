use dsp::{Node};

use clip_cubic;
use delay;
use fm;
use input;
use sine;

macro_rules! make_wrapper {
    (
        $name: ident,
        $($typ: ty => $wrap: ident),*
    ) => {
        #[derive(Debug)]
        pub enum $name {
            Blank,
            $($wrap($typ)),*
        }
        
        impl Node<[f32; 2]> for $name {
            fn audio_requested(&mut self, buffer: &mut [[f32; 2]], sample_hz: f64) {
                match *self {
                    $name::Blank => (),
                    $( $name::$wrap(ref mut node) => node.audio_requested(buffer, sample_hz) ),*
                }
            }
            
            fn dry(&self) -> f32 {
                match *self {
                    $name::Blank => 0.0,
                    $( $name::$wrap(ref node) => node.dry() ),*
                }
            }
            
            fn wet(&self) -> f32 {
                match *self {
                    $name::Blank => 1.0,
                    $( $name::$wrap(ref node) => node.wet() ),*
                }
            }
        }
    }
}

make_wrapper!(DspNode,
    sine::Sine => Sine,
    input::Input => Input,
    delay::Delay => Delay,
    clip_cubic::ClipCubic => ClipCubic,
    fm::FmSynth => FmSynth
);
