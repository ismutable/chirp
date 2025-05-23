use jack::contrib::ClosureProcessHandler;
use jack::{AudioInPort, AudioOutPort, Client, ClientOptions, Control, ProcessScope, ClosureProcessHandler};
use std::f32::consts::PI;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use hound;

const DURATION_SECS: usize = 5;
const FREQUENCY: f32 = 22_500.0;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // open client, requires jackd to be already running
    let (client, _status) = Client::new("duplex", ClientOptions::NO_START_SERVER)?;

    // register input & output ports
    let mut out_port = client.register_port("out", jack::AudioOut::default())?;
    let in_port = client.register_port("in", jack::AudioIn)?;

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
    let phase = Arc::new(Mutex::new(0usize));

    // create async process closure
    let rec_cb = Arc::clone(&recorded);
    let table_cb = Arc::clone(&sine_table);
    let phase_cb = Arc::new(Mutex::new(0usize));
    let process = move |_: &Client, ps: &ProcessScope| -> Control {
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
            rec.push(in_buff[i]);
            // advance phase (wrapping)
            *ph = (idx + 1) % table_cb.len();

        };

        Control::Continue
    };

    // activate jack client
    let active_client = client.activate_async((), ClosureProcessHandler::new(process));
    println!("Running full-duplex for {} seconds...", DURATION_SECS);
    thread::sleep(Duration::from_secs(DURATION_SECS as u64));
    drop(active_client);
    println!("Full-duplex complete");

    // create wave-file specification
    let spec = hound::WaveSpec {
        channels: 1,
        sample_rate: sample_rate as u32,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int
    };

    // write pure tone
    println!("Saving d;ata/pure_tone.wav...");
    {
        let mut writer = hound::WavWriter::create("data/pure_tone.wav", spec)?;
        for idx in 0..(sample_rate * DURATION_SECS) {
            let ph = idx % sine_table.len();
            writer.write_sample((sine_table[ph] * i16::MAX as f32) as i16)?;
        }
    }
    println!("Save complete.");

    Ok(())
}
