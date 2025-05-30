use hound;
use std::{f32::consts::PI, i16};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // constants
    const SAMPLE_RATE: u32 = 48_000;
    const FREQUENCY: f32 = 22_500.0;
    const DURATION_SECS: f32 = 1.0;

    // create wave specification
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: SAMPLE_RATE,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    // derivatives
    let amplitude = i16::MAX as f32;
    let sample_count = (SAMPLE_RATE as f32 * DURATION_SECS) as usize;

    // write to file
    println!("Writing tone.wav");
    let mut writer = hound::WavWriter::create("data/tone.wav", spec)?;
    for n in 0..sample_count {
        let t = n as f32 / SAMPLE_RATE as f32;
        let sample = (amplitude * (2.0 * PI * FREQUENCY * t).sin()) as i16;
        writer.write_sample(sample)?;
    }
    writer.finalize()?;
    println!("Writing tone.wav done");
    Ok(())
}
