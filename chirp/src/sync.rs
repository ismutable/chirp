use rtrb::{Consumer, Producer, RingBuffer};

pub struct WaveChannel {}

impl WaveChannel {
    pub fn with_capacity<T>(capacity: usize) -> (WaveWriter<T>, WaveReader<T>) {
        let (producer, consumer) = RingBuffer::new(capacity);
        (WaveWriter { producer }, WaveReader { consumer })
    }
}

pub struct WaveWriter<T> {
    producer: Producer<T>,
}

pub struct WaveReader<T> {
    consumer: Consumer<T>,
}

impl <T> WaveWriter<T> {
    pub fn write<samples: &[T]> {
        todo!();
    }
}
