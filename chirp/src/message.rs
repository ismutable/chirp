use crate::error::ChirpError;
use crate::expand::expand_lsb;
use crate::{BitModulator, WaveReader, WaveWriter, FRAME};

pub trait WaveLen {
    fn wave_len(&self) -> usize;
}

impl WaveLen for &[u8] {
    fn wave_len(&self) -> usize {
        self.len() * 8 * FRAME
    }
}

pub fn modulate(bytes: &[u8], buffer: &mut [f32]) -> Result<(), ChirpError> {
    // confirm sufficient buffer
    let required = bytes.wave_len();
    let available = buffer.len();
    if available < required {
        return Err(ChirpError::Buffer {
            required,
            available,
        });
    }

    // modulation loop
    let mut writer = WaveWriter::from(buffer);
    let lookup = BitModulator::default();
    for byte in bytes {
        let bits = expand_lsb(*byte);
        for bit in bits {
            let mut reader = WaveReader::from(lookup.modulate(bit));
            writer.write(&mut reader);
        }
    }

    Ok(())
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn two_byte_message() {
        let msg = [0, 1];
        let mut buffer = [0.0; FRAME * 2 * 8];
        let lookup = BitModulator::default();
        let expected: Vec<f32> = [
            // msg[0]
            lookup.modulate(false),
            lookup.modulate(false),
            lookup.modulate(false),
            lookup.modulate(false),
            lookup.modulate(false),
            lookup.modulate(false),
            lookup.modulate(false),
            lookup.modulate(false),
            // msg[1]
            lookup.modulate(true),
            lookup.modulate(false),
            lookup.modulate(false),
            lookup.modulate(false),
            lookup.modulate(false),
            lookup.modulate(false),
            lookup.modulate(false),
            lookup.modulate(false),
        ]
        .into_iter()
        .flat_map(|v| v.to_owned())
        .collect();
        modulate(&msg, &mut buffer).expect("Buffer length insufficient.");
        assert_eq!(buffer.as_slice(), expected.as_slice());
    }

    #[test]
    #[should_panic]
    fn test_insufficient_buffer() {
        let msg = [1, 2, 3];
        let mut buffer = [0.0; FRAME];
        modulate(&msg, &mut buffer).expect("Should panic, buffer not long enough.")
    }
}
