use chirp_modem::CARRIER_SIGNAL;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};
fn main() -> anyhow::Result<()> {
    println!("--- chirp-modem ---");
    println!("CARRIER_SIGNAL: {:?}", &*CARRIER_SIGNAL);

    let host = cpal::default_host();

    let output_device = host
        .default_output_device()
        .expect("No output device available");
    let output_config = output_device
        .supported_output_configs()?
        .find(|cfg| {
            cfg.sample_format() == cpal::SampleFormat::F32
                && cfg.max_sample_rate().0 >= OUTPUT_SAMPLE_RATE_HZ
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

    println!("Playing modulated tone.");
    output_stream.play()?;
    thread::sleep(Duration::from_secs(3));
    
    println!("Closing program.")
    Ok(())
}
