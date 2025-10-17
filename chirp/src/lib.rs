use std::f32::consts::TAU;

const FRAME: usize = 32;
const STEP: usize = 13;
const SAMPLES: usize = FRAME;
const DEFAULT_HI_GAIN: f32 = 0.9;
const DEFAULT_LO_GAIN: f32 = 0.1;

pub struct BitModulator {
    unit: [f32; FRAME],
    hi: [f32; FRAME],
    lo: [f32; FRAME],
    bit: bool,
    cursor: usize,
    sample: usize,
}

impl Default for BitModulator {
    fn default() -> Self {
        let mut unit = [0.0; FRAME];
        let mut hi = [0.0; FRAME];
        let mut lo = [0.0; FRAME];
        let phase_step = TAU / FRAME as f32;
        for (idx, sample) in unit.iter_mut().enumerate() {
            *sample = f32::sin(idx as f32 * phase_step);
            hi[idx] = DEFAULT_HI_GAIN * (*sample);
            lo[idx] = DEFAULT_LO_GAIN * (*sample);
        }
        Self {
            unit,
            hi,
            lo,
            bit: false,
            cursor: 0,
            sample: 0,
        }
    }
}

impl BitModulator {
    pub fn hi_gain(&mut self, gain: f32) -> &mut Self {
        for (s, u) in self.hi.iter_mut().zip(self.unit.iter()) {
            *s = gain * u;
        }
        self
    }
    pub fn lo_gain(&mut self, gain: f32) -> &mut Self {
        for (s, u) in self.lo.iter_mut().zip(self.unit.iter()) {
            *s = gain * u;
        }
        self
    }
    pub fn bit(&mut self, bit: bool) -> &mut Self {
        self.bit = bit;
        self.cursor = 0;
        self.sample = 0;
        self
    }
}

impl Iterator for BitModulator {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.sample == SAMPLES {
            return None;
        }

        let lookup = if self.bit {
            self.hi.as_slice()
        } else {
            self.lo.as_slice()
        };

        let sample = lookup[self.cursor];
        self.cursor = (self.cursor + STEP) % FRAME;
        self.sample += 1;

        Some(sample)
    }
}
