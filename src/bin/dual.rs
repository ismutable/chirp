use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::error::Error;
use std::f32::consts::PI;
use std::sync::{Arc, Mutex};

fn main() -> Result<(), Box<dyn Error>> {
    let host = cpal::default_host();

    // === output stream (192 kHz playback) ===
    let output_device = host
        .default_output_device()
        .expect("No output device available");
    let output_config = output_device
        .supported_output_configs()?
        .find(|cfg| {
            cfg.sample_format() == cpal::SampleFormat::F32 && cfg.max_sample_rate().0 >= 192000
        })
        .expect("No supported 192 kHz output config.")
        .with_sample_rate(cpal::SampleRate(192000));
    let sample_rate_out = output_config.sample_rate().0 as f32;
    dbg!(sample_rate_out);

    let mut t = 0.0;
    let freq = 19_200.0;
    dbg!(freq);

    let output_stream = output_device.build_output_stream(
        &output_config.config(),
        move |data: &mut [f32], _| {
            for sample in data.iter_mut() {
                *sample = (2.0 * PI * freq * t).sin() * 0.2;
                t += 1.0 / sample_rate_out;
            }
        },
        |err| eprintln!("output error: {:?}", err),
        None,
    )?;

    output_stream.play()?;

    // === Let it run ===
    println!("Streaming audio.");
    std::thread::sleep(std::time::Duration::from_secs(3));

    println!("Exiting.");
    Ok(())
}
