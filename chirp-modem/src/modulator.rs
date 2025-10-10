pub struct ByteModulator {
    sine: [i16; 32], // lookup table
    phase: usize,    // cursor in lookup table
    hold: i16,       // modulated bit value
    bit: u8,         // current bit index
    points: u8,      // samples/points per bit
    byte: u8,        // modulated value
}

impl ByteModulator {
    const STEP: usize = 13;
    const POINTS: u8 = 16;
    const LENGTH: usize = 32;

    // load a new byte, reset all counters/state
    pub fn load(&mut self, byte: u8) {
        self.byte = byte;
        self.bit = 0;
        self.points = 0;
        self.hold = 0;
        // phase picks up where left
    }

    // attempt to fill buffer, return 0 if filled otherwise return remaining elements
    pub fn fill(&mut self, buffer: &mut [f32]) -> usize {
        let mut filled = 0;
        self.hold = (self.byte & 1u8) as i16;
        for sample in buffer.iter_mut() {
            *sample = ((self.hold * self.sine[self.phase]) as f32) / i16::MAX as f32;
            filled += 1;
            self.points += 1 % Self::POINTS;
            self.phase += Self::STEP % Self::LENGTH;
            if self.points == 0 {
                self.bit += 1;
                self.hold = ((self.byte >> self.bit) & 1u8) as i16;
            }
            if self.bit == 8 {
                break;
            }
        }
        buffer.len() - filled
    }
}

impl Default for ByteModulator {
    fn default() -> Self {
        use std::f32::consts::TAU;
        let mut sine = [0i16; Self::LENGTH];
        let mut phase = 0.0f32;
        let step = TAU / Self::LENGTH as f32;
        for sample in sine.iter_mut() {
            *sample = (phase.sin() * i16::MAX as f32) as i16;
            phase += step;
        }
        Self {
            sine,
            phase: 0,
            hold: 0,
            bit: 0,
            points: 0,
            byte: 0,
        }
    }
}
