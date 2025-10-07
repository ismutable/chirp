use std::f32::consts::TAU;
use std::sync::LazyLock;

use bitvec::{
    order::Lsb0,
    slice::{BitSlice, Iter as BitIter},
    view::BitView,
};

pub type Hz = u16;

pub const BIT_REPEATS: u8 = 16;
pub const CARRIER_STEPS: u8 = 13;
pub const CARRIER_SAMPLES: u8 = 32;
pub const CARRIER_FREQ: Hz = 19_500;
pub const SAMPLE_RATE: Hz = 48_000;

pub static CARRIER_SIGNAL: LazyLock<Vec<i16>> = LazyLock::new(|| {
    let mut carrier = Vec::with_capacity(CARRIER_SAMPLES as usize);
    for i in 0..CARRIER_SAMPLES {
        let radian = ((i as f32) / (CARRIER_SAMPLES as f32)) * TAU;
        let scaled = radian.sin() * (i16::MAX as f32);
        let entry = scaled.clamp(i16::MIN as f32, i16::MAX as f32).round() as i16;
        carrier.push(entry);
    }
    carrier
});

pub struct WaveGenerator<'a> {
    one: i16,
    zero: i16,
    cursor: u8,
    count: u8,
    hold: bool,
    bits: BitIter<'a, u8, Lsb0>,
}

impl<'a> WaveGenerator<'a> {
    fn new(data: &'a [u8]) -> Self {
        let bits: &'a BitSlice<u8, Lsb0> = data.view_bits::<Lsb0>();
        Self {
            one: i16::MAX,
            zero: i16::MAX / 10,
            cursor: 0,
            count: 0,
            hold: false,
            bits: bits.iter(),
        }
    }
}

impl<'a> Iterator for WaveGenerator<'a> {
    type Item = i16;
    fn next(&mut self) -> Option<Self::Item> {
        if self.count == 0 {
            self.hold = *self.bits.next()?;
        }
        let value = CARRIER_SIGNAL[self.cursor as usize];
        self.count = (self.count + 1) % BIT_REPEATS;
        self.cursor = (self.cursor + CARRIER_STEPS) % CARRIER_SAMPLES;
        if self.hold {
            Some(self.one * value)
        } else {
            Some(self.zero * value)
        }
    }
}

pub struct WaveBuilder {}

impl WaveBuilder {
    pub fn generate(data: &[u8]) -> WaveGenerator<'_> {
        WaveGenerator::new(data)
    }
}
