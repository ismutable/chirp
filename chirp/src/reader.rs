pub trait WaveReader {
    fn remaining(&self) -> usize;
    fn read(&mut self, size: usize) -> &[f32];
    fn rewind(&mut self);
}

// private generic implementations
fn remaining<B: AsRef<[f32]>>(buffer: B, cursor: usize) -> usize {
    buffer.as_ref().len() - cursor
}

pub struct RefWaveReader<'m> {
    buffer: &'m [f32],
    pos: usize,
}

impl<'m> WaveReader for RefWaveReader<'m> {
    #[inline]
    fn remaining(&self) -> usize {
        remaining(self.buffer, self.pos)
    }

    fn read(&mut self, size: usize) -> &[f32] {
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

    fn rewind(&mut self) {
        self.pos = 0;
    }
}

impl<'m> From<&'m [f32]> for RefWaveReader<'m> {
    fn from(buffer: &'m [f32]) -> Self {
        RefWaveReader { buffer, pos: 0 }
    }
}
