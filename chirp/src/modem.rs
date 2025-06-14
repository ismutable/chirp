use std::f32::consts::PI;
use once_cell::sync::Lazy;

const TABLE_LENGTH: usize = 10;
const SAMPLE_RATE_HZ: u32 = 192_000;
const CYCLE_PER_BIT: usize = 40;

static SINE_TABLE: Lazy<[f32; TABLE_LENGTH]> = Lazy::new(|| {
    let mut table = [0.0; TABLE_LENGTH];
    for i in 0..TABLE_LENGTH {
        table[i] = (2.0 * PI * i as f32 / TABLE_LENGTH as f32).sin();
    }
    table
});

static COSINE_TABLE: Lazy<[f32; TABLE_LENGTH]> = Lazy::new(|| {
    let mut table = [0.0; TABLE_LENGTH];
    for i in 0..TABLE_LENGTH {
        table[i] = (2.0 * PI * i as f32 / TABLE_LENGTH as f32).cos();
    }
    table
});