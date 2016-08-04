use dsp::{FromSample, Sample};

use base::TAU;

pub fn sine<S: Sample>(phase: f64) -> S
    where S: Sample + FromSample<f32>,
{
    (phase.sin() as f32).to_sample::<S>()
}

pub fn step_phase(step: f64, phase: f64) -> f64 {
    (phase + step) % TAU
}
