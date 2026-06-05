use crate::lookup::SymbolTable;

struct Modulator<'a> {
    buffer: &'a mut [f32],
    symbol: SymbolTable,
    pos: usize,
    prior: Option<&'static [f32]>,
}

impl<'a> Default for Modulator<'a> {
    fn default() -> Self {
        Self {
            buffer: &mut [],
            symbol: SymbolTable::default(),
            pos: 0,
            prior: None,
        }
    }
}

impl<'a> Modulator<'a> {
    pub fn set_buffer(&mut self, buffer: &'a mut [f32]) -> usize {
        self.buffer = buffer;
        if let Some(prior) = self.prior.take() {
            self.write_static_slice(prior)
        } else {
            0
        }
    }

    pub fn remaining(&self) -> usize {
        self.buffer.len() - self.pos
    }

    pub fn is_full(&self) -> bool {
        self.buffer.len() == self.pos
    }

    fn write_static_slice(&mut self, src: &'static [f32]) -> usize {
        let length = self.remaining().min(src.len());
        let end = self.pos + length;
        self.buffer[self.pos..end].copy_from_slice(&src[..length]);
        self.prior = if length < src.len() {
            Some(&src[length..])
        } else {
            None
        };
        length
    }

    pub fn write_symbol(&mut self, bit: bool) -> usize {
        let symbol = if bit {
            self.symbol.one()
        } else {
            self.symbol.zero()
        };
        let symbol: &'static [f32] =
            unsafe { std::mem::transmute::<&[f32], &'static [f32]>(symbol) };
        self.write_static_slice(symbol)
    }
}
