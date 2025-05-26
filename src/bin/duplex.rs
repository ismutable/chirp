use hound;
use jack::contrib::ClosureProcessHandler;
use jack::{Client, ClientOptions, Control, ProcessScope};
use std::f32::consts::PI;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

const DURATION_SECS: usize = 10;
const FREQUENCY: f32 = 22_500.0;

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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Initializing client.");
    // open client, requires jackd to be already running
    let (client, _status) = Client::new("duplex", ClientOptions::NO_START_SERVER)?;

    // register input & output ports
    let mut out_port = client.register_port("out", jack::AudioOut::default())?;
    let in_port = client.register_port("in", jack::AudioIn::default())?;

    // build thread safe sine table abstractions and recording buffer
    let sample_rate = client.sample_rate();
    let recorded = Arc::new(Mutex::new(Vec::<f32>::with_capacity(
        DURATION_SECS * sample_rate,
    )));
    let sine_table: Arc<Vec<f32>> = Arc::new(
        (0..sample_rate)
            .map(|n| ((2.0 * PI * FREQUENCY * (n as f32 / sample_rate as f32)).sin()))
            .collect(),
    );

    let is_connected = Arc::new(Mutex::new(false));

    // create async process closure
    let process = {
        let rec_cb = Arc::clone(&recorded);
        let table_cb = Arc::clone(&sine_table);
        let phase_cb = Arc::new(Mutex::new(0usize));
        let is_connected = Arc::clone(&is_connected);
        move |client: &Client, ps: &ProcessScope| -> Control {
            // connect to system ports (run-once)
            if !*is_connected.lock().unwrap() {
                let playback_names: Vec<_> = client
                    .ports(Some("system"), None, jack::PortFlags::IS_INPUT)
                    .into_iter()
                    .collect();
                let capture_names: Vec<_> = client
                    .ports(None, None, jack::PortFlags::IS_OUTPUT)
                    .into_iter()
                    .collect();
                if let Some(play_name) = playback_names.get(0) {
                    if let Some(play_port) = client.port_by_name(play_name) {
                        client.connect_ports(&out_port, &play_port).unwrap();
                    }
                }
                if let Some(capt_name) = capture_names.get(0) {
                    if let Some(capt_port) = client.port_by_name(capt_name) {
                        client.connect_ports(&capt_port, &in_port).unwrap();
                    }
                }
            }
            *is_connected.lock().unwrap() = true;

            // duplex callback
            let out_buf = out_port.as_mut_slice(ps);
            let in_buf = in_port.as_slice(ps);
            let mut rec = rec_cb.lock().unwrap();
            let mut ph = phase_cb.lock().unwrap();
            for (i, out_sample) in out_buf.iter_mut().enumerate() {
                // write sample from sine table
                let idx = *ph;
                let s = table_cb[idx];
                *out_sample = s;
                // capture recorded sample
                rec.push(in_buf[i]);
                // advance phase (wrapping)
                *ph = (idx + 1) % table_cb.len();
            }

            Control::Continue
        }
    };

    // activate jack client
    let active = client.activate_async((), ClosureProcessHandler::new(process))?;
    println!("Running full-duplex for {} seconds...", DURATION_SECS);
    thread::sleep(Duration::from_secs(DURATION_SECS as u64));
    drop(active);
    println!("Full-duplex complete");

    // create wave-file specification
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: sample_rate as u32,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    // write pure tone to disk
    println!("Saving data/pure_tone.wav...");
    {
        let mut writer = hound::WavWriter::create("data/pure_tone.wav", spec)?;
        for idx in 0..(sample_rate * DURATION_SECS) {
            let ph = idx % sine_table.len();
            writer.write_sample((sine_table[ph] * i16::MAX as f32) as i16)?;
        }
    }
    println!("Save complete.");

    // write microphone recording to disk
    println!("Saving data/recorded.wav...");
    {
        let mut rec = recorded.lock().unwrap();
        let mut writer = hound::WavWriter::create("data/recorded.wav", spec)?;
        normalize_wave((*rec).as_mut_slice());
        for &sample in rec.iter() {
            writer.write_sample((sample * i16::MAX as f32) as i16)?;
        }
        writer.finalize()?;
    }

    println!("Save complete.");

    Ok(())
}
