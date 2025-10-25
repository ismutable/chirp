use anyhow::Context;
use chirp::expand::expand_lsb;
use chirp::{WaveReader, WaveWriter};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{BufferSize, ChannelCount, SampleFormat, SampleRate, StreamConfig};

use chirp::BitModulator;

use std::time::Duration;

const SAMPLE_FORMAT: SampleFormat = SampleFormat::F32;
const SAMPLE_RATE: SampleRate = SampleRate(48_000);
const CHANNELS: ChannelCount = 1;
const SECONDS: Duration = Duration::from_secs(3);

fn main() -> anyhow::Result<()> {
    // use default hardware
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .context("No default output device.")?;

    // validate audio device is capable of configuration
    device
        .supported_output_configs()?
        .filter(|cfg| cfg.sample_format() == SAMPLE_FORMAT && cfg.channels() == CHANNELS)
        .find(|cfg| cfg.min_sample_rate() >= SAMPLE_RATE && SAMPLE_RATE <= cfg.max_sample_rate())
        .context("No available configs match request audio settings.")?;

    // build config
    let config = StreamConfig {
        channels: CHANNELS,
        sample_rate: SAMPLE_RATE,
        buffer_size: BufferSize::Default,
    };

    // message to wave
    let msg = [0b10101010; 1 << 12];
    let bit = BitModulator::default();
    let wave: Vec<_> = msg
        .into_iter()
        .flat_map(expand_lsb)
        .flat_map(|b| bit.modulate(b))
        .cloned()
        .collect();

    // TODO: need to make reader variant that uses Arc so it can be Send + 'static
    let mut reader = WaveReader::from(wave.as_slice());

    // configure stream
    let stream = device.build_output_stream(
        &config,
        move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            let mut writer = WaveWriter::from(data);
            //writer.write(&mut reader);
            // if reader.remaining() == 0 {
            //     reader.rewind();
            // }
        },
        |_| panic!("Audio hardware refused config."),
        None,
    )?;

    stream.play()?;

    std::thread::sleep(SECONDS);
    Ok(())
}
