use std::f32::consts::TAU;
use std::sync::LazyLock;

use bitvec::order::Lsb0;
use bitvec::slice::BitSlice;
use bitvec::slice::Iter as BitIter;
use bitvec::view::AsBits;
use bitvec::view::BitView;

pub type Hz = u16;

pub const BIT_REPEATS: u8 = 16;
pub const CARRIER_STEPS: usize = 13;
pub const CARRIER_SAMPLES: usize = 32;
pub const CARRIER_FREQ: Hz = 19_500;
pub const SAMPLE_RATE: Hz = 48_000;

pub static CARRIER_SIGNAL: LazyLock<Vec<i16>> = LazyLock::new(|| {
    let mut carrier = Vec::with_capacity(CARRIER_SAMPLES);
    for i in 0..CARRIER_SAMPLES {
        let radian = ((i as f32) / (CARRIER_SAMPLES as f32)) * TAU;
        let scaled = radian.sin() * (i16::MAX as f32);
        let entry = scaled.clamp(i16::MIN as f32, i16::MAX as f32).round() as i16;
        carrier.push(entry);
    }
    carrier
});

pub struct WaveGenerator<'a> {
    cursor: u8,
    count: u8,
    bits: BitIter<'a, u8, Lsb0>,
}

impl<'a> WaveGenerator<'a> {
    fn new(data: &'a [u8]) -> Self {
        let bits: &'a BitSlice<u8, Lsb0> = data.view_bits::<Lsb0>();
        Self {
            cursor: 0,
            count: 0,
            bits: bits.iter(),
        }
    }
}

// impl<'a> Iterator for WaveGenerator<'a> {
//     type Item = i16;

//     fn next(&mut self) -> Option<Self::Item> {
//         if self.iter.is_none() {
//             self.iter = Some(self.data.as_bits().into_iter());
//         }
//         Some(0)
//     }
// }

pub struct WaveBuilder {}

impl WaveBuilder {
    pub fn generate(data: &[u8]) -> WaveGenerator<'_> {
        WaveGenerator::new(data)
    }
}
