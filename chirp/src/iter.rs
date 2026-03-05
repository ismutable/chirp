use std::iter::{ExactSizeIterator, Iterator};
use std::slice::Iter as SliceIter;

pub struct ByteIter {
    byte: u8,
    pos: u8,
}

impl ByteIter {
    pub fn new(byte: u8) -> Self {
        Self { byte, pos: 0 }
    }
}

impl Iterator for ByteIter {
    type Item = bool;

    fn next(&mut self) -> Option<bool> {
        // exit early if all bits have been vended
        if self.pos as u32 == u8::BITS {
            return None;
        }
        // return current lsb
        let bit = (self.byte & 1u8) == 1;
        // shift over one and track pos
        self.byte >>= 1;
        self.pos += 1;
        // return bit as bool
        Some(bit)
    }

    fn size_hint(&self) -> (usize, std::option::Option<usize>) {
        (0, Some(u8::BITS as usize))
    }
}

impl ExactSizeIterator for ByteIter {}

pub struct ByteSliceIter<'a> {
    bytes: SliceIter<'a, u8>,
    bits: ByteIter,
}

impl<'a> ByteSliceIter<'a> {
    pub fn new(packet: &'a [u8]) -> Self {
        let mut bytes = packet.iter();
        let bits = if let Some(byte) = bytes.next() {
            ByteIter::new(*byte)
        } else {
            // guard against empty array being passed
            // should never happen, prevents nested options
            let mut bits = ByteIter::new(0);
            for _ in 0..u8::BITS {
                bits.next();
            }
            bits
        };

        Self { bytes, bits }
    }
}

impl<'a> Iterator for ByteSliceIter<'a> {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        // vend from current byte iter until complete
        if let Some(bit) = self.bits.next() {
            return Some(bit);
        }

        // get next byte in packet; if exists
        if let Some(byte) = self.bytes.next() {
            self.bits = ByteIter::new(*byte);
            return self.bits.next();
        }

        // if both iterator expended, return None
        None
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.bytes.len() * u8::BITS as usize))
    }
}

impl<'a> ExactSizeIterator for ByteSliceIter<'a> {}
