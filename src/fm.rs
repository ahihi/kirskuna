use dsp::{Frame, Node, slice};
use portmidi::{MidiEvent};

use base::{TAU};
use math;
use midi::{MidiDestination};
use sine::Sine;

#[derive(Debug)]
pub enum AdState {
    Off,
    Attack,
    Decay
}

#[derive(Debug)]
pub struct AdEnvelope {
    state: AdState,
    pos: f64,
    attack: f64,
    decay: f64
}

impl AdEnvelope {
    pub fn new(attack: f64, decay: f64) -> AdEnvelope {
        AdEnvelope {
            state: AdState::Off,
            pos: 0.0,
            attack: attack,
            decay: decay
        }
    }
    
    pub fn value(&self) -> f32 {
        match self.state {
            AdState::Off    => 0.0,
            AdState::Attack => self.pos as f32,
            AdState::Decay  => 1.0 - self.pos as f32
        }
    }
    
    pub fn step(&mut self, sample_hz: f64) {
        match self.state {
            AdState::Off    => {},
            AdState::Attack => {
                self.pos += self.attack / sample_hz;
                if 1.0 <= self.pos {
                    self.state = AdState::Decay;
                    self.pos = 0.0;
                }
            },
            AdState::Decay  => {
                self.pos += self.decay / sample_hz;
                if 1.0 <= self.pos {
                    self.state = AdState::Off;
                    self.pos = 0.0;
                }
            }
        }
    }
    
    pub fn trigger(&mut self) {
        self.state = AdState::Attack;
        self.pos = 0.0; 
    }
}

#[derive(Debug)]
pub struct Operator {
    base_frequency: f64,
    sine: Sine,
    amp_env: AdEnvelope
}

impl Operator {
    pub fn new(base_frequency: f64, amplitude: f32, amp_env: AdEnvelope) -> Operator {
        Operator {
            base_frequency: base_frequency,
            sine: Sine {
                frequency: base_frequency,
                phase: 0.0,
                amplitude: amplitude
            },
            amp_env: amp_env
        }
    }
    
    pub fn value(&self) -> f32 {
        self.sine.value() * self.amp_env.value()
    }
    
    pub fn step(&mut self, sample_hz: f64) {
        self.sine.step(sample_hz);
        self.amp_env.step(sample_hz);
    }
}

#[derive(Debug)]
pub struct FmSynth {
    pub carrier: Operator,
    pub modulator: Operator
}

impl FmSynth {
    pub fn trigger(&mut self) {
        self.carrier.amp_env.trigger();
        self.modulator.amp_env.trigger();
    }
}

impl Node<[f32; 2]> for FmSynth {
    fn audio_requested(&mut self, buffer: &mut [[f32; 2]], sample_hz: f64) {
        slice::map_in_place(buffer, |_| {
            let output = self.carrier.value();
            self.carrier.sine.frequency = self.carrier.base_frequency + self.modulator.value() as f64;
            self.carrier.step(sample_hz);
            self.modulator.step(sample_hz);
            
            Frame::from_fn(|_| output)
        });
    }
}

impl MidiDestination for FmSynth {
    fn process_events(&mut self, events: &[MidiEvent]) {
        println!("{:?}", events);
    }
}
