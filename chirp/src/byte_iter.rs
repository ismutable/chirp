struct Byter(u8);

impl From<u8> for Byter {
    fn from(byte: u8) -> Self {
        Self(byte)
    }
}

// TODO: better than into iter because size is known as compile time, always prefer slice iters
impl From<Byter> for [bool; 8] {
    fn from(mut byter: Byter) -> Self {
        let mut output = [false; 8];
        for bit in output.iter_mut() {
            *bit = (byter.0 & 1u8) == 1;
            byter.0 >>= 1;
        }
        output
    }
}
