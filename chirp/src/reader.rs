use std::sync::Arc;

pub trait WaveReader {
    fn remaining(&self) -> usize;
    fn read(&mut self, size: usize) -> &[f32];
    fn rewind(&mut self);
}

// private generic implementations
#[inline]
fn remaining<B: AsRef<[f32]> + ?Sized>(buffer: &B, cursor: usize) -> usize {
    buffer.as_ref().len() - cursor
}

fn read<'b, B: AsRef<[f32]> + ?Sized>(buffer: &'b B, cursor: &mut usize, size: usize) -> &'b [f32] {
    let buffer = buffer.as_ref();

    if size > remaining(buffer, *cursor) {
        let output = &buffer[*cursor..];
        *cursor = buffer.len();
        output
    } else {
        let stop = *cursor + size;
        let output = &buffer[*cursor..stop];
        *cursor = stop;
        output
    }
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
