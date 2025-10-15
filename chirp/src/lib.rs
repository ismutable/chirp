use std::f32::consts::TAU;

const SAMPLES: usize = 32;
const DEFAULT_HI_GAIN: f32 = 0.9;
const DEFAULT_LO_GAIN: f32 = 0.1;

pub struct BitModulator {
    unit: [f32; SAMPLES],
    hi: [f32; SAMPLES],
    lo: [f32; SAMPLES],
}

impl Default for BitModulator {
    fn default() -> Self {
        let mut unit = [0.0; SAMPLES];
        let mut hi = [0.0; SAMPLES];
        let mut lo = [0.0; SAMPLES];
        let step = TAU / SAMPLES as f32;
        for (idx, sample) in unit.iter_mut().enumerate() {
            *sample = f32::sin(idx as f32 * step);
            hi[idx] = DEFAULT_HI_GAIN * (*sample);
            lo[idx] = DEFAULT_LO_GAIN * (*sample);
        }
        Self { unit, hi, lo }
    }
}

impl BitModulator {
    fn hi_gain(&mut self, gain: f32) -> &mut Self {
        self
    }
}
