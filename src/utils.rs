
pub fn combine_bytes(upper: u16, lower: u16) -> u16 {
    return (upper << 8) | lower;
}

pub fn get_top_bit(val: u8) -> bool {
    return (val >> 7) == 1;
}

pub fn check_bit(val: u8, bit: u8) -> bool {
    return (val >> (bit - 1)) & 1 == 1;
}

// ref http://www.righto.com/2012/12/the-6502-overflow-flag-explained.html
pub fn is_overflow(res: u8, m: u8, n: u8) -> bool {
    return ((m ^ res) & (n ^ res) & 0x80) != 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_bit_false() {
        assert!(!check_bit(0xF0, 2));
    }

    #[test]
    fn test_check_bit_true() {
        assert!(check_bit(0xF0, 6));
    }

    #[test]
    fn test_is_overflow_false() {
        assert!(!is_overflow(224, 80, 140));
    }


    #[test]
    fn test_is_overflow_true() {
        assert!(is_overflow(160, 80, 80));
    }

    #[test]
    fn test_combine_bytes() {
        let lower: u16 = 0x56;
        let upper: u16 = 0x34;
        assert_eq!(combine_bytes(upper, lower), 0x3456);
    }

    #[test]
    fn test_get_top_bit_true() {
        assert!(get_top_bit(0xF1));
    }

    #[test]
    fn test_get_top_bit_false() {
        assert!(!get_top_bit(0x11));
    }
}