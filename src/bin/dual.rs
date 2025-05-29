use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::error::Error;
use std::f32::consts::PI;
use std::i16;
use std::sync::{Arc, Mutex};

const OUTPUT_SAMPLE_RATE_HZ: u32 = 192_000; // Primary Sound Card
const INPUT_SAMPLE_RATE_HZ: u32 = 96_000; // Built-In USB Mic
const SESSION_DURATION_SEC: u32 = 5;
const FREQUENCY_HZ: u32 = 440;

fn normalize_wave(values: &mut [f32]) {
    let &min = values
        .iter()
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let &max = values
        .iter()
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let mid = (min + max) / 2.0;
    let half_range = (max - min) / 2.0;
    for val in values.iter_mut() {
        *val = (*val - mid) / half_range;
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let host = cpal::default_host();

    // === output stream (192 kHz playback) ===
    let output_device = host
        .default_output_device()
        .expect("No output device available");
    let output_config = output_device
        .supported_output_configs()?
        .find(|cfg| {
            cfg.sample_format() == cpal::SampleFormat::F32 && cfg.max_sample_rate().0 >= OUTPUT_SAMPLE_RATE_HZ
        })
        .expect("No supported 192 kHz output config.")
        .with_sample_rate(cpal::SampleRate(OUTPUT_SAMPLE_RATE_HZ));
    let sample_rate_out = output_config.sample_rate().0 as f32;
    dbg!(sample_rate_out);

    let mut t = 0.0;
    let output_stream = output_device.build_output_stream(
        &output_config.config(),
        move |data: &mut [f32], _| {
            for sample in data.iter_mut() {
                *sample = (2.0 * PI * FREQUENCY_HZ as f32 * t).sin() * 0.2;
                t += 1.0 / sample_rate_out;
            }
        },
        |err| eprintln!("output error: {:?}", err),
        None,
    )?;

    output_stream.play()?;

    // === Input Stream (96 kHz recording) ===
    let input_device = host
        .default_input_device()
        .expect("No input device available.");
    let input_config = input_device
        .supported_input_configs()?
        .find(|c| {
            c.sample_format() == cpal::SampleFormat::F32 && c.max_sample_rate().0 >= INPUT_SAMPLE_RATE_HZ
        })
        .expect("No supported 96 kHz input config.")
        .with_sample_rate(cpal::SampleRate(INPUT_SAMPLE_RATE_HZ));

    let recorded_samples = Arc::new(Mutex::new(Vec::<f32>::new()));
    let input_buf = Arc::clone(&recorded_samples);

    let input_stream = input_device.build_input_stream(
        &input_config.config(),
        move |data: &[f32], _| {
            let mut buf = input_buf.lock().unwrap();
            buf.extend_from_slice(data);
        },
        |err| eprint!("Input error: {:?}", err),
        None
    )?;

    input_stream.play()?;

    // === Let it run ===
    println!("Streaming audio.");
    std::thread::sleep(std::time::Duration::from_secs(SESSION_DURATION_SEC as u64));

    // create wave-file specifications
    let output_spec = hound::WavSpec {
        channels: 1,
        sample_rate: OUTPUT_SAMPLE_RATE_HZ,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let input_spec = hound::WavSpec {
        channels: 1,
        sample_rate: INPUT_SAMPLE_RATE_HZ,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    // write pure tone to disk
    println!("Saving data/dual_pure_tone.wav...");
    {
        let mut writer = hound::WavWriter::create("data/dual_pure_tone.wav", output_spec)?;
        let mut t = 0.0;
        for _ in 0..(OUTPUT_SAMPLE_RATE_HZ * SESSION_DURATION_SEC) {
            let v = ((2.0 * PI * FREQUENCY_HZ as f32 * t).sin() * i16::MAX as f32) as i16;
            t += 1.0 / OUTPUT_SAMPLE_RATE_HZ as f32;
            writer.write_sample(v)?
        }
        writer.finalize()?
    }
    println!("Save complete.");

    // write microphone recording to disk
    println!("Saving data/dual_recorded.wav...");
    {
        let mut rec = recorded_samples.lock().unwrap();
        let mut writer = hound::WavWriter::create("data/dual_recorded.wav", input_spec)?;
        normalize_wave((*rec).as_mut_slice());
        for &v in rec.iter() {
            writer.write_sample((v * i16::MAX as f32) as i16)?;
        }
        writer.finalize()?;
    }
    println!("Save complete.");

    println!("Exiting.");
    Ok(())
}
