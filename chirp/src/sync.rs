use rtrb::{chunks::ChunkError, Consumer, CopyToUninit, Producer, RingBuffer};

use crate::{error::ChirpError, message::WaveLen, WaveWriter};

pub struct WaveChannel {}

impl WaveChannel {
    pub fn with_capacity<T>(capacity: usize) -> (WaveSender<T>, WaveReceiver<T>) {
        let (producer, consumer) = RingBuffer::new(capacity);
        (WaveSender { producer }, WaveReceiver { consumer })
    }
}

pub struct WaveSender<T> {
    producer: Producer<T>,
}

pub struct WaveReceiver<T> {
    consumer: Consumer<T>,
}

impl<T> WaveSender<T> {
    pub fn modulate(&mut self, bytes: &[u8]) -> Result<(), ChirpError> {
        // confirm sufficient buffer
        let required = bytes.wave_len();
        let available = self.producer.slots();
        if available < required {
            return Err(ChirpError::Buffer {
                required,
                available,
            });
        }

        // get chunk interface and adapt 3rd part lib error
        let chunk = self.producer.write_chunk_uninit(required).map_err(|e| {
            if let ChunkError::TooFewSlots(available) = e {
                ChirpError::Buffer {
                    required,
                    available,
                }
            } else {
                unreachable!("No other ChunkError variants");
            }
        })?;

        let (first, second) = chunk.as_mut_slices();

        // modulation loop
        let mut writer = Some(WaveWriter::from(first));
        let lookup = BitModulator::default();
        for byte in bytes {
            let bits = expand_lsb(*byte);
            for bit in bits {
                let mut reader = WaveReader::from(lookup.modulate(bit));
                writer.write(&mut reader);
                // when writer full swap with new instance using second buffer, then check if there is data remaining
                // in the current reader before moving to the next bit
            }
        }

        Ok(())
    }
}
