use std::{f32::consts::PI, io::Cursor};
use bitstream_io::{BitReader, BitRead, BigEndian};


struct LazyWaveform{
    sample_rate: u32,
    carrier_freq: u32,
    lo_sine: Vec<f32>,
    hi_sine: Vec<f32>,
}

impl LazyWaveform {
    pub fn new(sample_rate: u32, carrier_freq: u32, lo_gain: f32, hi_gain: f32) -> Self {
        // allocate memory
        let samples = (sample_rate / carrier_freq) as usize;
        let mut lo_sine = Vec::with_capacity(samples);
        let mut hi_sine = Vec::with_capacity(samples);

        // populate static sine wave lookup tables
        for i in 1..=samples {
            let t = i as f32 / sample_rate as f32;
            let v = (2.0 * PI * carrier_freq as f32 * t).sin(); // FIXME: doesn't scale freq
            lo_sine.push(lo_gain * v);
            hi_sine.push(hi_gain * v);
        }
        // build lazy waveform generator
        Self {
            sample_rate,
            carrier_freq,
            lo_sine,
            hi_sine,
        }
    }

    pub fn modulate(data: &[u8]) -> impl Iterator<Item=f32> + '_{
        let bits: BitReader<_, BigEndian> = BitReader::new(Cursor::new(data));
        bits.flat_map(move |bit| {
            let wave = if bit { one_wave } else { zero_wave };
            std::iter::repeat(wave.iter().copied()).take(repeat).flatten()
        })
    }
}

struct LazyModulator<'a, 'b> {
    data: BitReader<'a, BigEndian>,
    lo_sine: &'b [f32],
    hi_sine: &'b [f32],
}

impl <'a, 'b> LazyModulator<'a, 'b> {
    const REPEAT: usize = 1 << 5; // power of 2 for efficient modulo
    pub fn new(data: &'a [u8], lo_sine: &'b [f32], hi_sine: &'b [f32], repeats: u128) -> Self {
        Self {
            data: BitReader::new(Cursor::new(data)),
            lo_sine,
            hi_sine,
        }
    }
}

impl Iterator for LazyModulator<'_, '_> {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
    }
}