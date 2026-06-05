//! Symbol Table
//!
//! sine() cannot be called at compile-time. This class allows the sine-based symbol
//! waveforms to be calculated one at instatiation for O(1) waveform generation

use std::f32::consts::TAU;

const SAMPLES: usize = 32;
const STEP: usize = 7;
const ONE_GAIN: f32 = 0.9;
const ZERO_GAIN: f32 = 0.1;

#[derive(Debug, Clone)]
pub struct SymbolTable {
    one: [f32; SAMPLES],
    zero: [f32; SAMPLES],
}

impl Default for SymbolTable {
    fn default() -> Self {
        let mut one = [0.0; SAMPLES];
        let mut zero = [0.0; SAMPLES];
        let phase_step = (TAU / SAMPLES as f32) * STEP as f32;
        for idx in 0..SAMPLES {
            let sample = f32::sin(idx as f32 * phase_step);
            one[idx] = ONE_GAIN * sample;
            zero[idx] = ZERO_GAIN * sample;
        }
        Self { one, zero }
    }
}

impl SymbolTable {
    const SAMPLES: usize = SAMPLES;

    pub fn one(&self) -> &[f32] {
        &self.one
    }

    pub fn zero(&self) -> &[f32] {
        &self.zero
    }
}
