use std::f32::consts::TAU;
use std::sync::LazyLock;

pub type Hz = u16;

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
