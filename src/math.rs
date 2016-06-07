use dsp::{FromSample, Sample};

pub fn sine<S: Sample>(phase: f64) -> S
    where S: Sample + FromSample<f32>,
{
    (phase.sin() as f32).to_sample::<S>()
}