use std::{error::Error, sync::mpsc};

const HI_GAIN: f32 = 0.9;
const LO_GAIN: f32 = 0.1;
const WAVE_LEN: usize = 32;
const BUF_LEN: usize = WAVE_LEN * u8::BITS as usize;
const STEP_SIZE: usize = 13;
const TAU: f32 = std::f32::consts::TAU;
static mut HI_WAVE: [f32; WAVE_LEN] = [0.0; WAVE_LEN];
static mut LO_WAVE: [f32; WAVE_LEN] = [0.0; WAVE_LEN];

fn init_wave(wave: &mut [f32], gain: f32) {
    if wave.len() < WAVE_LEN {
        panic!("Target wave not big enough");
    }
    let phase_step = (TAU / WAVE_LEN as f32) * STEP_SIZE as f32;
    for idx in 0..WAVE_LEN {
        wave[idx] = gain * f32::sin(idx as f32 * phase_step);
    }
}

fn write_byte(buffer: &mut [f32], byte: u8) {
    if buffer.len() < BUF_LEN {
        panic!("Target buffer not big enough.");
    };

    for bit in 0..u8::BITS as usize {
        let bit_val = 1 & (byte << bit);
        let wave_ref = unsafe {
            if bit_val == 1 {
                &HI_WAVE
            } else {
                &LO_WAVE
            }
        };

        buffer[(bit * WAVE_LEN)..((bit + 1) * WAVE_LEN)].copy_from_slice(wave_ref);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    unsafe {
        init_wave(&mut HI_WAVE, HI_GAIN);
        init_wave(&mut LO_WAVE, LO_GAIN);
    }

    let (tx, rx) = mpsc::channel::<Vec<u8>>();

    let mut current: Option<<std::vec::Vec<u8> as std::iter::IntoIterator>::IntoIter> =
        Some(vec![].into_iter());

    let closure = move |data: &mut [f32]| {
        // only write full byte(s)
        if data.len() < BUF_LEN {
            return;
        }

        // TODO: could this be a match expression, separate byte writing from reload logic
        // ie try multiple ways to produce an Option<u8>, if there is a current
        // current might not be necessary with vec![].into_iter()., this will always product none
        // when next is called and remove a layer of wrapping
        // we are close to a simple declarative version
        // if there is an active message
        if let Some(bytes) = &mut current {
            // write the next byte to audio buffer
            if let Some(byte) = bytes.next() {
                write_byte(data, byte);
            } else {
                // if msg is complete set current to none
                current = None;
            }
        } else if let Ok(bytes) = rx.try_recv() {
            current = Some(bytes.into_iter());
        } else {
            // write zeros, most common background state
            todo!();
        }
    };
    let msg = "Hello World".as_bytes().to_vec();
    tx.send(msg);
    Ok(())
}
