//! Oscillator components.

use super::audionode::*;
use super::math::*;
use super::signal::*;
use super::*;
use numeric_array::*;
use std::marker::PhantomData;

/// Sine oscillator.
pub struct Sine<T: Float> {
    _marker: PhantomData<T>,
    phase: f64,
    sample_duration: f64,
    hash: u64,
}

impl<T: Float> Sine<T> {
    pub fn new(sample_rate: f64) -> Sine<T> {
        Sine {
            _marker: PhantomData,
            phase: 0.0,
            sample_duration: 1.0 / sample_rate,
            hash: 0,
        }
    }
}

impl<T: Float> AudioNode for Sine<T> {
    const ID: u64 = 21;
    type Sample = T;
    type Inputs = typenum::U1;
    type Outputs = typenum::U1;

    fn reset(&mut self, sample_rate: Option<f64>) {
        self.phase = rnd(self.hash as i64);
        if let Some(sr) = sample_rate {
            self.sample_duration = 1.0 / sr
        };
    }

    #[inline]
    fn tick(
        &mut self,
        input: &Frame<Self::Sample, Self::Inputs>,
    ) -> Frame<Self::Sample, Self::Outputs> {
        let frequency = input[0].to_f64();
        self.phase = (self.phase + frequency * self.sample_duration).fract();
        [convert(sin(self.phase * TAU))].into()
    }

    #[inline]
    fn set_hash(&mut self, hash: u64) {
        self.hash = hash;
        self.reset(None);
    }

    fn route(&self, _input: &SignalFrame, _frequency: f64) -> SignalFrame {
        let mut output = new_signal_frame(self.outputs());
        output[0] = Signal::Latency(0.0);
        output
    }
}
