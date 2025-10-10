struct ByteModulator {
    sine: [i16; 32], // lookup table
    phase: usize,    // cursor in lookup table
    bit: u8,         // current bit
    samples: u8,     // samples per bit
    byte: u8,        // modulated value
}
