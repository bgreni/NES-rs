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
    // ADD ops
    AdcI = 0x69,
    AdcZp = 0x65,
    AdcZpX = 0x75,
    AdcAbs = 0x6D,
    AdcAbsX = 0x7D,
    AdcAbsY = 0x79,
    AdcIndX = 0x61,
    AdcIndY = 0x71,

    // AND ops
    AndI = 0x29,
    AndZp = 0x25,
    AndZpX = 0x35,
    AndAbs = 0x2D,
    AndAbsX = 0x3D,
    AndAbsY = 0x39,
    AndIndX = 0x21,
    AndIndY = 0x31,

    // ASL ops
    AslAccum = 0x0A,
    AslZp = 0x06,
    AslZpX = 0x16,
    AslAbs = 0x0E,
    AslAbsX = 0x1E,

    // BRANCH ops
    Bcc = 0x90,
    Bcs = 0xB0,
    Beq = 0xF0,
    Bne = 0xD0,
    Bmi = 0x30,
    Bpl = 0x10,
    Bvc = 0x50,
    Bvs = 0x70,

    // BIT check ops
    BitZp = 0x24,
    BitAbs = 0x2C,

    // Force interrupt
    Brk = 0x00,

    // Clear flags
    Clc = 0x18,
    Cld = 0xD8,
    Cli = 0x58,
    Clv = 0xB8,

    // Compare
    CmpI = 0xC9,
    CmpZp = 0xC5,
    CmpZpX = 0xD5,
    CmpAbs = 0xCD,
    CmpAbsX = 0xDD,
    CmpAbsY = 0xD9,
    CmpIndX = 0xC1,
    CmpIndY = 0xD1,

    // Compare X

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