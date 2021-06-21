use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

pub struct Instruction {
    op: u8,
}

#[derive(Debug)]
pub enum AddrModes {
    Immediate,
    ZeroPage,
    ZeroPageX,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    IndirectX,
    IndirectY,
    Accumulator
}

#[derive(FromPrimitive, PartialEq)]
pub enum Ops {
    AdcI = 0x69,
    AdcZp = 0x65,
    AdcZpX = 0x75,
    AdcAbs = 0x6D,
    AdcAbsX = 0x7D,
    AdcAbsY = 0x79,
    AdcIndX = 0x61,
    AdcIndY = 0x71,

    AndI = 0x29,
    AndZp = 0x25,
    AndZpX = 0x35,
    AndAbs = 0x2D,
    AndAbsX = 0x3D,
    AndAbsY = 0x39,
    AndIndX = 0x21,
    AndIndY = 0x31,

    AslAccum = 0x0A,
    AslZp = 0x06,
    AslZpX = 0x16,
    AslAbs = 0x0E,
    AslAbsX = 0x1E,

    Bcc = 0x90,
    Bcs = 0xB0,
    Beq = 0xF0,
    Bmi = 0x30,

    BitZp = 0x24,
    BitAbs = 0x2C,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_int() {
        let op: Ops = FromPrimitive::from_u8(0x69).unwrap();
        assert!(op == Ops::AdcI);
    }
}