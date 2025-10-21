use crate::error::ChirpError;
use crate::expand::expand_lsb;
use crate::{BitModulator, WaveReader, WaveWriter, FRAME};

trait WaveLen {
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
