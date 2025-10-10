pub mod modulator;

use std::f32::consts::TAU;
use std::sync::LazyLock;

use bitvec::{
    order::Lsb0,
    slice::{BitSlice, Iter as BitIter},
    view::BitView,
};

pub type Hz = u32;

pub const BIT_REPEATS: u8 = 16; // lines up w/ zero crossing of sine wave
pub const CARRIER_STEPS: u8 = 13; // prime ensures every element using mod
pub const CARRIER_SAMPLES: u8 = 32; // rational divisor based on prime and sample_rate/carrier freq ratio
pub const CARRIER_FREQ: Hz = 19_500; // low-end ultrasonic, meets nyquist criteria with sample rate
pub const SAMPLE_RATE: Hz = 48_000; // average stock sound card sampling rate:w

// look-up table to avoid sine computation in-the-loop
pub static CARRIER_SIGNAL: LazyLock<Vec<f32>> = LazyLock::new(|| {
    let mut carrier = Vec::with_capacity(CARRIER_SAMPLES as usize);
    for i in 0..CARRIER_SAMPLES {
        let radian = ((i as f32) / (CARRIER_SAMPLES as f32)) * TAU;
        let scaled = radian.sin() * f32::MAX;
        carrier.push(scaled);
    }
    carrier
});

/// zero-copy iterator signal modulator
pub struct WaveGenerator<'a> {
    one: f32,
    zero: f32,
    cursor: u8,
    count: u8,
    hold: bool,
    bits: BitIter<'a, u8, Lsb0>,
}

impl<'a> WaveGenerator<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        let bits: &'a BitSlice<u8, Lsb0> = data.view_bits::<Lsb0>();
        Self {
            one: f32::MAX,         // place-holder, will be dynamic
            zero: f32::MAX / 10.0, // place-holder, will be dynamic
            cursor: 0,
            count: 0,
            hold: false,
            bits: bits.iter(),
        }
    }
}

impl<'a> Iterator for WaveGenerator<'a> {
    type Item = f32;
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
