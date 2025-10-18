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

struct WaveReader<'m> {
    buffer: &'m [f32],
    pos: usize,
}

impl<'m> WaveReader<'m> {
    #[inline]
    pub fn remaining(&self) -> usize {
        self.buffer.len() - self.pos
    }

    pub fn read(&mut self, size: usize) -> &[f32] {
        if size > self.remaining() {
            let output = &self.buffer[self.pos..];
            self.pos = self.buffer.len();
            output
        } else {
            let stop = self.pos + size;
            let output = &self.buffer[self.pos..stop];
            self.pos = stop;
            output
        }
    }
}

impl<'m> From<&'m [f32]> for WaveReader<'m> {
    fn from(buffer: &'m [f32]) -> Self {
        WaveReader { buffer, pos: 0 }
    }
}

struct WaveWriter<'m> {
    buffer: &'m mut [f32],
    pos: usize,
}

impl<'m> From<&'m mut [f32]> for WaveWriter<'m> {
    fn from(buffer: &'m mut [f32]) -> Self {
        WaveWriter { buffer, pos: 0 }
    }
}

type ReaderIterMut<'c, 'b> = std::slice::IterMut<'c, WaveReader<'b>>;

impl<'m> WaveWriter<'m> {
    #[inline]
    pub fn remaining(&self) -> usize {
        self.buffer.len() - self.pos
    }

    pub fn write(&mut self, reader: &mut WaveReader) {
        let data = reader.read(self.remaining());
        self.buffer.copy_from_slice(data);
        self.pos += data.len();
    }

    pub fn batch_write<'c, 'b>(&mut self, readers: &mut ReaderIterMut<'c, 'b>) {
        for reader in readers {
            // writer buffer full
            if self.remaining() == 0 {
                break;
            }
            // current reader empty
            if reader.remaining() == 0 {
                continue;
            }
            // write has vacancy and reader has data
            self.write(reader)
        }
    }
}
