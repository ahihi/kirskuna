use dsp::{FromSample, Sample};

pub fn sine<S: Sample>(phase: f64) -> S
    where S: Sample + FromSample<f32>,
{
    use std::f64::consts::PI;
    ((phase * PI * 2.0).sin() as f32).to_sample::<S>()
}