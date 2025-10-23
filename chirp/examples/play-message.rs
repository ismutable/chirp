use anyhow::Context;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{BufferSize, ChannelCount, SampleFormat, SampleRate, StreamConfig};

const SAMPLE_FORMAT: SampleFormat = SampleFormat::F32;
const SAMPLE_RATE: SampleRate = SampleRate(48_000);
const CHANNELS: ChannelCount = 1;

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

    // configure stream
    // TODO: impl data_callback using chirp lib
    let stream = device.build_output_stream(&config, todo!(), |e| panic!("Audio hardware refused config."), None)

    Ok(())
}
