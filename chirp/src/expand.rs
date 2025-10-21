#[inline]
pub fn expand_lsb(mut byte: u8) -> [bool; 8] {
    let mut output = [false; 8];
    for bit in output.iter_mut() {
        *bit = (byte & 1u8) == 1;
        byte >>= 1;
    }
    output
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(0b0000_0000, [false, false, false, false, false, false, false, false])]
    #[case(0b0000_0001, [true, false, false, false, false, false, false, false])]
    #[case(0b1000_0000, [false, false, false, false, false, false, false, true])]
    #[case(0b0101_0101, [true, false, true, false, true, false, true, false])]
    fn test_expand_lsb(#[case] input: u8, #[case] output: [bool; 8]) {
        assert_eq!(expand_lsb(input), output);
    }
}
