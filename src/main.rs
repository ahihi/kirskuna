extern crate dsp;
extern crate portaudio;

extern crate kirskuna;

use dsp::{slice, Graph, Node, NodeIndex};
use dsp::sample::ToFrameSliceMut;
use portaudio as pa;

use kirskuna::base::{Output, CHANNELS};
use kirskuna::clip_cubic::{ClipCubic};
use kirskuna::delay::{Delay};
use kirskuna::input::{Input};
use kirskuna::node::{DspNode};
//use kirskuna::sine::{Sine};

const FRAMES: u32 = 64;
const SAMPLE_HZ: f64 = 44_100.0;

const IN_CHANNELS: usize = 6;
const IN_LEFT: usize = 2;
const IN_RIGHT: usize = 3;

fn main() {
    run().unwrap()
}

fn run() -> Result<(), pa::Error> {
    let mut graph = Graph::new();

    let master = graph.add_node(DspNode::Blank);
    
    let mut inputs: Vec<NodeIndex> = Vec::new();
    
    /*let (_, delay) = graph.add_input(
        DspNode::Delay(Delay::new(SAMPLE_HZ as usize)),
        master
    );*/
    
    let (_, clip_cubic) = graph.add_input(
        DspNode::ClipCubic(ClipCubic::new(8.0, 0.1, 1.0, 1.0)),
        master
    );
    
    /*let (_, _sine) = graph.add_input(
        DspNode::Sine(Sine { frequency: 440.0, phase: 0.0, amplitude: 0.5 }),
        delay
    );*/
    
    let (_, input) = graph.add_input(
        DspNode::Input(Input::new(FRAMES as usize)),
        clip_cubic
    );
    inputs.push(input);

    graph.set_master(Some(master));

    let mut elapsed: f64 = 0.0;
    let mut prev_time = None;

    let callback = move |pa::stream::DuplexCallbackArgs { in_buffer, out_buffer, time, .. }| {
        for &input_ix in &inputs {
            if let Some(&mut DspNode::Input(ref mut input_node)) = graph.node_mut(input_ix) {
                let frames = in_buffer.chunks(IN_CHANNELS).map(|frame| [frame[IN_LEFT], frame[IN_RIGHT]]);
                input_node.fill(frames);
            }
        }
        
        let out_buffer: &mut [[Output; CHANNELS]] = out_buffer.to_frame_slice_mut().unwrap();
        slice::equilibrium(out_buffer);
        graph.audio_requested(out_buffer, SAMPLE_HZ);

        let last_time = prev_time.unwrap_or(time.current);
        let dt = time.current - last_time;
        elapsed += dt;
        prev_time = Some(time.current);

        pa::Continue
    };

    let pa = try!(pa::PortAudio::new());
        
    let settings = try!(pa.default_duplex_stream_settings::<Output, Output>(IN_CHANNELS as i32, CHANNELS as i32, SAMPLE_HZ, FRAMES));
    
    let mut stream = try!(pa.open_non_blocking_stream(settings, callback));
    try!(stream.start());

    while let true = try!(stream.is_active()) {
        ::std::thread::sleep(::std::time::Duration::from_millis(16));
    }

    Ok(())
}
