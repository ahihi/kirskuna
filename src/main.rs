extern crate dsp;
extern crate portaudio;

extern crate sixier;

use dsp::{slice, Graph, Node};
use dsp::sample::ToFrameSliceMut;
use portaudio as pa;

use sixier::base::{Output, CHANNELS};
use sixier::node::{DspNode};
use sixier::sine::{Sine};

const FRAMES: u32 = 64;
const SAMPLE_HZ: f64 = 44_100.0;

fn main() {
    run().unwrap()
}

fn run() -> Result<(), pa::Error> {
    let mut graph = Graph::new();

    let master = graph.add_node(DspNode::Blank);
    
    graph.add_input(DspNode::Sine(Sine { frequency: 440.0, phase: 0.0, amplitude: 0.5 }), master);

    graph.set_master(Some(master));

    let mut elapsed: f64 = 0.0;
    let mut prev_time = None;

    let callback = move |pa::OutputStreamCallbackArgs { buffer, time, .. }| {
        let buffer: &mut [[Output; CHANNELS]] = buffer.to_frame_slice_mut().unwrap();
        slice::equilibrium(buffer);
        graph.audio_requested(buffer, SAMPLE_HZ);

        let last_time = prev_time.unwrap_or(time.current);
        let dt = time.current - last_time;
        elapsed += dt;
        prev_time = Some(time.current);

        // Traverse inputs or outputs of a node with the following pattern.
        /*
        let mut inputs = graph.inputs(synth);
        while let Some(input_idx) = inputs.next_node(&graph) {
            if let DspNode::Oscillator(_, ref mut pitch, _) = graph[input_idx] {
                // Pitch down our oscillators for fun.
                *pitch -= 0.1;
            }
        }
        */

        //if timer >= 0.0 { pa::Continue } else { pa::Complete }
        pa::Continue
    };

    let pa = try!(pa::PortAudio::new());
    let settings = try!(pa.default_output_stream_settings::<Output>(CHANNELS as i32, SAMPLE_HZ, FRAMES));
    
    let mut stream = try!(pa.open_non_blocking_stream(settings, callback));
    try!(stream.start());

    while let true = try!(stream.is_active()) {
        ::std::thread::sleep(::std::time::Duration::from_millis(16));
    }

    Ok(())
}

/*impl Node<Output> for DspNode {
    /// Here we'll override the audio_requested method and generate a sine wave.
    fn audio_requested(&mut self, buffer: &mut [Output], settings: Settings) {
        match *self {
            DspNode::Synth => (),
            DspNode::Kick { frequency, bend_rate, ref pattern, ref mut state, .. } => {
                for frame in buffer.chunks_mut(settings.channels as usize) {
                    let steps = pattern.len();
                    
                    let step_ix: usize = state.turn.floor() as usize % steps;

                    let atk: f64 = 0.01;
                    let turn_fract: f64 = state.turn.fract();
                    let amp: f64 = if pattern[step_ix] {
                        if turn_fract < atk {
                            turn_fract / atk
                        } else {
                            (1.0 - turn_fract - atk) / (1.0 - atk)
                        }
                    } else {
                        0.0
                    };
                    let sine: f32 = sine_wave(state.phase, 0.3);
                    let sample = amp as f32 * sine;
                    
                    for channel in frame.iter_mut() {
                        *channel = sample;
                    }
                    state.turn += TEMPO as f64 / (60.0 * settings.sample_hz as f64);
                    state.phase += frequency / settings.sample_hz as f64;
                }
            }
            
            /*
            DspNode::Oscillator(ref mut phase, frequency, volume) => {
                for frame in buffer.chunks_mut(settings.channels as usize) {
                    let val = sine_wave(*phase, volume);
                    for channel in frame.iter_mut() {
                        *channel = val;
                    }
                    *phase += frequency / settings.sample_hz as f64;
                }
            },
            */
        }
    }
}*/
