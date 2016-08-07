extern crate dsp;
extern crate getopts;
extern crate portaudio;
extern crate portmidi;

extern crate kirskuna;

use std::cmp;
use std::env;
use std::error::Error;

use dsp::{slice, Graph, Node, NodeIndex};
use dsp::sample::ToFrameSliceMut;
use getopts::Options;
use portaudio as pa;
use portmidi::{PortMidi, InputPort};

use kirskuna::base::{Output};
use kirskuna::clip_cubic::{ClipCubic};
use kirskuna::fm::{AdEnvelope, FmSynth, Operator};
use kirskuna::delay::{Delay};
use kirskuna::input::{Input};
use kirskuna::midi::{MidiDestination};
use kirskuna::node::{DspNode};
//use kirskuna::sine::{Sine};

const SAMPLE_HZ: f64 = 44_100.0;

struct RunOptions {
    buf_size: usize,
    midi_buf_size: usize,
    in_channels: usize,
    in_left: usize,
    in_right: usize,
    out_channels: usize,
    out_left: usize,
    out_right: usize
}

enum Command {
    Help(String),
    Run(RunOptions)
}

fn main() {
    match get_command().unwrap() {
        Command::Help(text)     => println!("{}", text),
        Command::Run(run_opts)  => run(&run_opts).unwrap()
    }
}

fn get_usage(program: &str, opts: Options) -> String {
    let brief = format!("Usage: {} [options]", program);
    format!("{}", opts.usage(&brief))
}

fn get_command() -> Result<Command, Box<Error>> {
    let mut opts = Options::new();
    
    opts.optflag("h", "help", "print help");
    opts.optopt("b", "buf-size", "buffer size", "FRAMES");
    opts.optopt("B", "midi-buf-size", "MIDI buffer size", "EVENTS");
    opts.optopt("l", "in-left", "left input channel", "CHANNEL");
    opts.optopt("r", "in-right", "right input channel", "CHANNEL");
    //opts.optopt("m", "midi-in", "MIDI input device index", "DEVICE");
    opts.optopt("L", "out-left", "left output channel", "CHANNEL");
    opts.optopt("R", "out-right", "right output channel", "CHANNEL");
        
    let args: Vec<String> = env::args().collect();
    
    let m = try!(opts.parse(&args));
    
    if m.opt_present("h") {
        return Ok(Command::Help(get_usage(&args[0], opts)));
    }
        
    let buf_size = match m.opt_str("b") {
        Some(s) => try!(s.parse::<usize>()),
        None    => 64
    };
    
    let midi_buf_size = match m.opt_str("B") {
        Some(s) => try!(s.parse::<usize>()),
        None    => 256
    };
    
    let in_left = match m.opt_str("l") {
        Some(s) => try!(s.parse::<usize>()),
        None    => 0
    };

    let in_right = match m.opt_str("r") {
        Some(s) => try!(s.parse::<usize>()),
        None    => 1
    };
    
    let in_channels = cmp::max(in_left, in_right) + 1;

    let out_left = match m.opt_str("L") {
        Some(s) => try!(s.parse::<usize>()),
        None    => 0
    };

    let out_right = match m.opt_str("R") {
        Some(s) => try!(s.parse::<usize>()),
        None    => 1
    };

    let out_channels = cmp::max(out_left, out_right) + 1;

    Ok(Command::Run(RunOptions {
        buf_size: buf_size,
        midi_buf_size: midi_buf_size,
        in_channels: in_channels,
        in_left: in_left,
        in_right: in_right,
        out_channels: out_channels,
        out_left: out_left,
        out_right: out_right
    }))
}

fn run(opts: &RunOptions) -> Result<(), Box<Error>> {    
    let buf_size = opts.buf_size;
    let midi_buf_size = opts.midi_buf_size;
    let in_left = opts.in_left;
    let in_right = opts.in_right;
    let in_channels = opts.in_channels;
    let out_left = opts.out_left;
    let out_right = opts.out_right;
    let out_channels = opts.out_channels;
    
    let mut graph = Graph::new();

    let master = graph.add_node(DspNode::Blank);
    
    let mut inputs: Vec<NodeIndex> = Vec::new();
    
    /*let (_, delay) = graph.add_input(
        DspNode::Delay(Delay::new(SAMPLE_HZ as usize)),
        master
    );*/
    
    /*let (_, clip_cubic) = graph.add_input(
        DspNode::ClipCubic(ClipCubic::new(8.0, 0.1, 1.0, 1.0)),
        master
    );*/
    
    /*let (_, _sine) = graph.add_input(
        DspNode::Sine(Sine { frequency: 440.0, phase: 0.0, amplitude: 0.5 }),
        delay
    );*/
    
    /*let (_, input) = graph.add_input(
        DspNode::Input(Input::new(buf_size)),
        clip_cubic
    );
    inputs.push(input);*/

    let mut fm = FmSynth {
        carrier: Operator::new(0.0, 0.5, AdEnvelope::new(128.0, 1.0)),
        modulator: Operator::new(0.0, 1000.0, AdEnvelope::new(128.0, 4.0))
    };
    
    let (_, fm_synth) = graph.add_input(
        DspNode::FmSynth(fm),
        master
    );

    graph.set_master(Some(master));

    let midi = try!(PortMidi::new());
    let midi_in = try!(midi.default_input_port(midi_buf_size));

    let mut elapsed: f64 = 0.0;
    let mut prev_time = None;

    let callback = move |pa::stream::DuplexCallbackArgs { in_buffer, out_buffer, time, .. }| {
        if let Ok(Some(midi_events)) = midi_in.read_n(midi_buf_size) {
            let mut visit_order = graph.visit_order();
            while let Some(node_ix) = visit_order.next(&graph) {
                if let Some(ref mut dsp_node) = graph.node_mut(node_ix) {
                    dsp_node.process_events(&midi_events);
                }
            }
            
        }
                
        for &input_ix in &inputs {
            if let Some(&mut DspNode::Input(ref mut input_node)) = graph.node_mut(input_ix) {
                let frames = in_buffer.chunks(in_channels).map(|frame| [frame[in_left], frame[in_right]]);
                input_node.fill(frames);
            }
        }
        
        let out_buffer: &mut [[Output; 2]] = out_buffer.to_frame_slice_mut().unwrap();
        slice::equilibrium(out_buffer);
        graph.audio_requested(out_buffer, SAMPLE_HZ);

        let last_time = prev_time.unwrap_or(time.current);
        let dt = time.current - last_time;
        elapsed += dt;
        prev_time = Some(time.current);

        pa::Continue
    };

    let pa = try!(pa::PortAudio::new());
        
    let settings = try!(pa.default_duplex_stream_settings::<Output, Output>(in_channels as i32, out_channels as i32, SAMPLE_HZ, buf_size as u32));
    
    let mut stream = try!(pa.open_non_blocking_stream(settings, callback));
    try!(stream.start());

    while let true = try!(stream.is_active()) {
        ::std::thread::sleep(::std::time::Duration::from_millis(16));
    }

    Ok(())
}
