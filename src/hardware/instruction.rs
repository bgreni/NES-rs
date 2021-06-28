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
    ZeroPageY,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    IndirectX,
    IndirectY,
    Accumulator,
    Indirect
}

#[derive(PartialEq)]
pub enum TransferOption {
    X,
    Y,
    A,
    S
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
    CmpXI = 0xE0,
    CmpXZp = 0xE4,
    CmpXAbs = 0xEC,

    // Compare Y
    CmpYI = 0xC0,
    CmpYZp = 0xC4,
    CmpYAbs = 0xCC,

    // Decrement
    DecZp = 0xC6,
    DecZpX = 0xD6,
    DecAbs = 0xCE,
    DecAbsX = 0xDE,
    DecX = 0xCA,
    DecY = 0x88,

    // Increment
    IncZp = 0xE6,
    IncZpX = 0xF6,
    IncAbs = 0xEE,
    IncAbsX = 0xFE,
    IncX = 0xE8,
    IncY = 0xC8,

    // XOR
    EORI = 0x49,
    EORZp = 0x45,
    EORZpX = 0x55,
    EORAbs = 0x4D,
    EORAbsX = 0x5D,
    EORAbsY = 0x59,
    EORIndX = 0x41,
    EORIndY = 0x51,

    // Jump
    JmpAbs = 0x4C,
    JmpInd = 0x6C,
    Jsr = 0x20,

    // Load Accum
    LdaI = 0xA9,
    LdaZp = 0xA5,
    LdaZpX = 0xB5,
    LdaAbs = 0xAD,
    LdaAbsX = 0xBD,
    LdaAbsY = 0xB9,
    LdaIndX = 0xA1,
    LdaIndY = 0xB1,

    LdxI = 0xA2,
    LdxZp = 0xA6,
    LdxZpY = 0xB6,
    LdxAbs = 0xAE,
    LdxAbsY = 0xBE,

    LdyI = 0xA0,
    LdyZp = 0xA4,
    LdyZpX = 0xB4,
    LdyAbs = 0xAC,
    LdyAbsX = 0xBC,

    // LSR ops
    LsrAccum = 0x4A,
    LsrZp = 0x46,
    LsrZpX = 0x56,
    LsrAbs = 0x4E,
    LsrAbsX = 0x5E,

    // NOOP
    Nop = 0xEA,

    // OR
    ORI = 0x09,
    ORZp = 0x05,
    ORZpX = 0x15,
    ORAbs = 0x0D,
    ORAbsX = 0x1D,
    ORAbsY = 0x19,
    ORIndX = 0x01,
    ORIndY = 0x11,

    // Push Accum
    Pha = 0x48,
    // Push status
    Php = 0x08,

    // Pull accum and status
    Pla = 0x68,
    Plp = 0x28,

    // ROL ops
    RolAccum = 0x2A,
    RolZp = 0x26,
    RolZpX = 0x36,
    RolAbs = 0x2E,
    RolAbsX = 0x3E,

    // ROR ops
    RorAccum = 0x6A,
    RorZp = 0x66,
    RorZpX = 0x76,
    RorAbs = 0x6E,
    RorAbsX = 0x7E,

    // Return from interrupt/subroutine
    Rti = 0x40,
    Rts = 0x60,

    // Sub
    SbcI = 0xE9,
    SbcZp = 0xE5,
    SbcZpX = 0xF5,
    SbcAbs = 0xED,
    SbcAbsX = 0xFD,
    SbcAbsY = 0xF9,
    SbcIndX = 0xE1,
    SbcIndY = 0xF1,

    // Set flags
    Sec = 0x38,
    Sed = 0xF8,
    Sei = 0x78,

    // Store registers
    StaZp = 0x85,
    StaZpX = 0x95,
    StaAbs = 0x8D,
    StaAbsX = 0x9D,
    StaAbsY = 0x99,
    StaIndX = 0x81,
    StaIndY = 0x91,

    StxZp = 0x86,
    StxZpY = 0x96,
    StxAbs = 0x8E,

    StyZp = 0x84,
    StyZpX = 0x94,
    StyAbs = 0x8C,

    // transfers
    Tax = 0xAA,
    Tay = 0xA8,
    Tsx = 0xBA,
    Txa = 0x8A,
    Txs = 0x9A,
    Tya = 0x98,
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