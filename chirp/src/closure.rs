use crate::{iter::SerialIter, BitModulator};
use std::sync::mpsc::{self, Sender};

fn factory() -> (Sender<Vec<u8>>, impl FnMut(&mut [f32])) {
    let (tx, rx) = mpsc::channel();
    let modulator = BitModulator::default();
    let serializer = SerialIter::new(&[]);

    let mut closure = move |data: &mut [f32]| {
        todo!();
    };
    (tx, closure)
}
