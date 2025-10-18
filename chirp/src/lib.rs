use std::f32::consts::TAU;

/// lifetimes
///
/// 'm: modulator lookup
/// 'b: output buffer
/// 'c: iterator container

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
    pub fn modulate(&self, bit: bool) -> &[f32] {
        if bit {
            &self.hi
        } else {
            &self.lo
        }
    }
}

pub struct WaveReader<'m> {
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

pub struct WaveWriter<'b> {
    buffer: &'b mut [f32],
    pos: usize,
}

impl<'b> From<&'b mut [f32]> for WaveWriter<'b> {
    fn from(buffer: &'b mut [f32]) -> Self {
        WaveWriter { buffer, pos: 0 }
    }
}

type ReaderIterMut<'c, 'm> = std::slice::IterMut<'c, WaveReader<'m>>;

impl<'b> WaveWriter<'b> {
    #[inline]
    pub fn remaining(&self) -> usize {
        self.buffer.len() - self.pos
    }

    pub fn write(&mut self, reader: &mut WaveReader) {
        let data = reader.read(self.remaining());
        let stop = self.pos + data.len();
        self.buffer[self.pos..stop].copy_from_slice(data);
        self.pos = stop;
    }

    pub fn batch_write<'c, 'm>(&mut self, readers: &mut ReaderIterMut<'c, 'm>) {
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

#[cfg(test)]
mod modulate {
    use super::*;

    #[test]
    fn equal_length() {
        let mut dst = [0.0; FRAME];
        let mut writer = WaveWriter::from(dst.as_mut_slice());
        let bit = BitModulator::default();
        let mut reader = WaveReader::from(bit.modulate(true));
        writer.write(&mut reader);
        assert_eq!(0, reader.remaining(), "Reader should have zero remaining.");
        assert_eq!(0, writer.remaining(), "Writer should have zero remaining.");
        assert_eq!(
            dst.as_slice(),
            bit.modulate(true),
            "Dst should be identical to Src"
        );
    }

    #[test]
    fn larger_output() {
        let mut dst = [0.0; FRAME + 1];
        let mut writer = WaveWriter::from(dst.as_mut_slice());
        let bit = BitModulator::default();
        let mut reader = WaveReader::from(bit.modulate(true));
        writer.write(&mut reader);
        assert_eq!(0, reader.remaining(), "Reader should have zero remaining.");
        assert_eq!(1, writer.remaining(), "Writer should have one remaining.");
        assert_eq!(
            &dst[..dst.len() - 1], // up to but not including last element
            bit.modulate(true),
            "Dst should match Src except for last element."
        );
        assert_eq!(
            0.0,
            dst[dst.len() - 1], // get last element
            "Last Dst element should equal default initialized value."
        );
    }

    #[test]
    fn larger_input() {
        let mut dst = [0.0; FRAME - 1];
        let mut writer = WaveWriter::from(dst.as_mut_slice());
        let bit = BitModulator::default();
        let lookup = bit.modulate(true);
        let mut reader = WaveReader::from(lookup);
        writer.write(&mut reader);
        assert_eq!(1, reader.remaining(), "Reader should have one remaining.");
        assert_eq!(0, writer.remaining(), "Writer should have zero remaining.");
        assert_eq!(
            dst.as_slice(),
            &lookup[..lookup.len() - 1],
            "Dst should be full and contain all but last element of Src."
        )
    }

    #[test]
    fn batch_write_multiple() {
        let bit = BitModulator::default()
        let init_reader = || WaveReader::from(bit.modulate(true));
        let readers = &[init_reader(), init_reader()];
        // TODO: left off here, testing batch writes over various length combinations
    }
}
