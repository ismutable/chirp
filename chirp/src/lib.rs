use std::f32::consts::TAU;

const FRAME: usize = 32;
const STEP: usize = 13;
const HI_GAIN: f32 = 1.0;
const LO_GAIN: f32 = 0.1;

#[derive(Debug, Clone)]
pub struct BitModulator {
    hi: [f32; FRAME],
    lo: [f32; FRAME],
}

impl Default for BitModulator {
    fn default() -> Self {
        let mut hi = [0.0; FRAME];
        let mut lo = [0.0; FRAME];
        let phase_step = (TAU / FRAME as f32) * STEP as f32;
        for idx in 0..FRAME {
            let sample = f32::sin(idx as f32 * phase_step);
            hi[idx] = HI_GAIN * sample;
            lo[idx] = LO_GAIN * sample;
        }
        Self { hi, lo }
    }
}

impl BitModulator {
    pub fn signal(&self, bit: bool) -> &[f32] {
        if bit {
            &self.hi
        } else {
            &self.lo
        }
    }
}

struct WaveReader<'a> {
    data: &'a [f32],
    pos: usize,
}

impl<'a> WaveReader<'a> {
    pub fn remaining(&self) -> usize {
        self.data.len() - self.pos
    }

    pub fn read(&mut self, size: usize) -> &[f32] {
        if size > self.remaining() {
            let output = &self.data[self.pos..];
            self.pos = self.data.len();
            output
        } else {
            let stop = self.pos + size;
            let output = &self.data[self.pos..stop];
            self.pos = stop;
            output
        }
    }
}

impl<'a> From<&'a [f32]> for WaveReader<'a> {
    fn from(data: &'a [f32]) -> Self {
        WaveReader { data, pos: 0 }
    }
}

struct WaveWriter<'a> {
    buffer: &'a mut [f32],
    pos: usize,
}

impl<'a> From<&'a mut [f32]> for WaveWriter<'a> {
    fn from(buffer: &'a mut [f32]) -> Self {
        WaveWriter { buffer, pos: 0 }
    }
}

enum Status {
    Full,
    Remaining(usize),
}

impl<'a> WaveWriter<'a> {
    pub fn remaining(&self) -> usize {
        self.buffer.len() - self.pos
    }

    pub fn write(&mut self, reader: WaveReader) -> Status {
        // handle reader empty case
        if reader.remaining() == 0 {
            return Status::Remaining(self.remaining());
        };

        // handle buffer full case
        if self.remaining() == 0 {
            return Status::Full;
        };

        if self.remaining() > reader.remaining {}

        Status::Remaining(self.remaining())
    }
}
