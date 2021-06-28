use crate::hardware::registers::{Flags, Registers};
use crate::hardware::instruction::{AddrModes, Ops, TransferOption};
use crate::hardware::memory::{MEM_SIZE, MemoryOps};
use crate::utils::{get_top_bit, is_overflow, check_bit, combine_bytes};
use num_traits::FromPrimitive;


pub struct Cpu {
    pub (super) memory: Vec<u8>,
    rega: u8,
    pub (super) regx: u8,
    pub (super) regy: u8,
    pub (super) flags: Flags,
    // pub (super) stack: Vec<u8>,
    pub (super) pc: u16,
    cycles_taken: u8,
    pub (super) sp: usize,
}

impl Cpu {
    pub fn new() -> Self {
        return Cpu {
            memory: vec![0; MEM_SIZE],
            rega: 0,
            regx: 0,
            regy: 0,
            flags: Flags::new(),
            pc: 0,
            // stack: Vec::new(),
            cycles_taken: 0,
            sp: 0xFF,
        }
    }

    pub fn exec_instruction(&mut self, op: Ops) {
        match op {
            Ops::AdcI => self.adc(AddrModes::Immediate),
            Ops::AdcZp => self.adc(AddrModes::ZeroPage),
            Ops::AdcZpX => self.adc(AddrModes::ZeroPageX),
            Ops::AdcAbs => self.adc(AddrModes::Absolute),
            Ops::AdcAbsX => self.adc(AddrModes::AbsoluteX),
            Ops::AdcAbsY => self.adc(AddrModes::AbsoluteY),
            Ops::AdcIndX => self.adc(AddrModes::IndirectX),
            Ops::AdcIndY => self.adc(AddrModes::IndirectY),

            Ops::AndI => self.and(AddrModes::Immediate),
            Ops::AndZp => self.and(AddrModes::ZeroPage),
            Ops::AndZpX => self.and(AddrModes::ZeroPageX),
            Ops::AndAbs => self.and(AddrModes::Absolute),
            Ops::AndAbsX => self.and(AddrModes::AbsoluteX),
            Ops::AndAbsY => self.and(AddrModes::AbsoluteY),
            Ops::AndIndX => self.and(AddrModes::IndirectX),
            Ops::AndIndY => self.and(AddrModes::IndirectY),

            Ops::AslAccum => self.asl(AddrModes::Accumulator),
            Ops::AslZp => self.asl(AddrModes::ZeroPage),
            Ops::AslZpX => self.asl(AddrModes::ZeroPageX),
            Ops::AslAbs => self.asl(AddrModes::Absolute),
            Ops::AslAbsX => self.asl(AddrModes::AbsoluteX),

            Ops::Bcc | Ops::Bcs => self.bcc_or_bcs(op != Ops::Bcc),
            Ops::Beq | Ops::Bne => self.beq_or_bne(op == Ops::Beq),
            Ops::Bmi => self.bmi(),
            Ops::Bpl => self.bpl(),
            Ops::Bvs | Ops::Bvc => self.bvs_or_bvc(op == Ops::Bvs),

            Ops::BitAbs => self.bit(AddrModes::Absolute),
            Ops::BitZp => self.bit(AddrModes::ZeroPage),

            Ops::Brk => self.brk(),

            Ops::Clc => self.flags.carry = false,
            Ops::Cld => self.flags.decimal = false,
            Ops::Cli => self.flags.inter_disable = false,
            Ops::Clv => self.flags.overflow = false,

            Ops::CmpI => self.cmp(AddrModes::Immediate, Registers::A),
            Ops::CmpZp => self.cmp(AddrModes::ZeroPage, Registers::A),
            Ops::CmpZpX => self.cmp(AddrModes::ZeroPageX, Registers::A),
            Ops::CmpAbs => self.cmp(AddrModes::Absolute, Registers::A),
            Ops::CmpAbsX => self.cmp(AddrModes::AbsoluteX, Registers::A),
            Ops::CmpAbsY => self.cmp(AddrModes::AbsoluteY, Registers::A),
            Ops::CmpIndX => self.cmp(AddrModes::IndirectX, Registers::A),
            Ops::CmpIndY => self.cmp(AddrModes::IndirectY, Registers::A),
            Ops::CmpXI => self.cmp(AddrModes::Immediate, Registers::X),
            Ops::CmpXZp => self.cmp(AddrModes::ZeroPage, Registers::X),
            Ops::CmpXAbs => self.cmp(AddrModes::Absolute, Registers::X),
            Ops::CmpYI => self.cmp(AddrModes::Immediate, Registers::Y),
            Ops::CmpYZp => self.cmp(AddrModes::ZeroPage, Registers::Y),
            Ops::CmpYAbs => self.cmp(AddrModes::Absolute, Registers::Y),

            Ops::DecZp => self.dec(AddrModes::ZeroPage),
            Ops::DecZpX => self.dec(AddrModes::ZeroPageX),
            Ops::DecAbs => self.dec(AddrModes::Absolute),
            Ops::DecAbsX => self.dec(AddrModes::AbsoluteX),
            Ops::DecX => self.dec_reg(Registers::X),
            Ops::DecY => self.dec_reg(Registers::Y),

            Ops::EORI => self.eor(AddrModes::Immediate),
            Ops::EORZp => self.eor(AddrModes::ZeroPage),
            Ops::EORZpX => self.eor(AddrModes::ZeroPageX),
            Ops::EORAbs => self.eor(AddrModes::Absolute),
            Ops::EORAbsX => self.eor(AddrModes::AbsoluteX),
            Ops::EORAbsY => self.eor(AddrModes::AbsoluteY),
            Ops::EORIndX => self.eor(AddrModes::IndirectX),
            Ops::EORIndY => self.eor(AddrModes::IndirectY),

            Ops::IncZp => self.inc(AddrModes::ZeroPage),
            Ops::IncZpX => self.inc(AddrModes::ZeroPageX),
            Ops::IncAbs => self.inc(AddrModes::Absolute),
            Ops::IncAbsX => self.inc(AddrModes::AbsoluteX),
            Ops::IncX => self.inc_reg(Registers::X),
            Ops::IncY => self.inc_reg(Registers::Y),

            Ops::JmpInd => self.jmp(AddrModes::Indirect),
            Ops::JmpAbs => self.jmp(AddrModes::Absolute),
            Ops::Jsr => self.jsr(),

            Ops::LdaI => self.ld_reg(AddrModes::Immediate, Registers::A),
            Ops::LdaZp => self.ld_reg(AddrModes::ZeroPage, Registers::A),
            Ops::LdaZpX => self.ld_reg(AddrModes::ZeroPageX, Registers::A),
            Ops::LdaAbs => self.ld_reg(AddrModes::Absolute, Registers::A),
            Ops::LdaAbsX => self.ld_reg(AddrModes::AbsoluteX, Registers::A),
            Ops::LdaAbsY => self.ld_reg(AddrModes::AbsoluteY, Registers::A),
            Ops::LdaIndX => self.ld_reg(AddrModes::IndirectX, Registers::A),
            Ops::LdaIndY => self.ld_reg(AddrModes::IndirectY, Registers::A),

            Ops::LdxI => self.ld_reg(AddrModes::Immediate, Registers::X),
            Ops::LdxZp => self.ld_reg(AddrModes::ZeroPage, Registers::X),
            Ops::LdxZpY => self.ld_reg(AddrModes::ZeroPageY, Registers::X),
            Ops::LdxAbs => self.ld_reg(AddrModes::Absolute, Registers::X),
            Ops::LdxAbsY => self.ld_reg(AddrModes::AbsoluteY, Registers::X),

            Ops::LdyI => self.ld_reg(AddrModes::Immediate, Registers::Y),
            Ops::LdyZp => self.ld_reg(AddrModes::ZeroPage, Registers::Y),
            Ops::LdyZpX => self.ld_reg(AddrModes::ZeroPageX, Registers::Y),
            Ops::LdyAbs => self.ld_reg(AddrModes::Absolute, Registers::Y),
            Ops::LdyAbsX => self.ld_reg(AddrModes::AbsoluteY, Registers::Y),

            Ops::LsrAccum => self.lsr(AddrModes::Accumulator),
            Ops::LsrZp => self.lsr(AddrModes::ZeroPage),
            Ops::LsrZpX => self.lsr(AddrModes::ZeroPageX),
            Ops::LsrAbs => self.lsr(AddrModes::Absolute),
            Ops::LsrAbsX => self.lsr(AddrModes::AbsoluteX),

            Ops::Nop => {},

            Ops::ORI => self.or(AddrModes::Immediate),
            Ops::ORZp => self.or(AddrModes::ZeroPage),
            Ops::ORZpX => self.or(AddrModes::ZeroPageX),
            Ops::ORAbs => self.or(AddrModes::Absolute),
            Ops::ORAbsX => self.or(AddrModes::AbsoluteX),
            Ops::ORAbsY => self.or(AddrModes::AbsoluteY),
            Ops::ORIndX => self.or(AddrModes::IndirectX),
            Ops::ORIndY => self.or(AddrModes::IndirectY),

            Ops::Pha => self.push_stack(self.rega),
            Ops::Php => self.save_status(),

            Ops::Pla => self.pull_register(true),
            Ops::Plp => self.pull_register(false),

            Ops::RolAccum => self.ror_or_rol(AddrModes::Accumulator, false),
            Ops::RolZp => self.ror_or_rol(AddrModes::ZeroPage, false),
            Ops::RolZpX => self.ror_or_rol(AddrModes::ZeroPageX, false),
            Ops::RolAbs => self.ror_or_rol(AddrModes::Absolute, false),
            Ops::RolAbsX => self.ror_or_rol(AddrModes::AbsoluteX, false),

            Ops::RorAccum => self.ror_or_rol(AddrModes::Accumulator, true),
            Ops::RorZp => self.ror_or_rol(AddrModes::ZeroPage, true),
            Ops::RorZpX => self.ror_or_rol(AddrModes::ZeroPageX, true),
            Ops::RorAbs => self.ror_or_rol(AddrModes::Absolute, true),
            Ops::RorAbsX => self.ror_or_rol(AddrModes::AbsoluteX, true),

            Ops::Rti | Ops::Rts => self.rti(),

            Ops::SbcI => self.sbc(AddrModes::Immediate),
            Ops::SbcZp => self.sbc(AddrModes::ZeroPage),
            Ops::SbcZpX => self.sbc(AddrModes::ZeroPageX),
            Ops::SbcAbs => self.sbc(AddrModes::Absolute),
            Ops::SbcAbsX => self.sbc(AddrModes::AbsoluteX),
            Ops::SbcAbsY => self.sbc(AddrModes::AbsoluteY),
            Ops::SbcIndX => self.sbc(AddrModes::IndirectX),
            Ops::SbcIndY => self.sbc(AddrModes::IndirectY),

            Ops::Sec => self.flags.carry = true,
            Ops::Sed => self.flags.decimal = true,
            Ops::Sei => self.flags.inter_disable = true,

            Ops::StaZp => self.store_reg(AddrModes::ZeroPage, Registers::A),
            Ops::StaZpX => self.store_reg(AddrModes::ZeroPageX, Registers::A),
            Ops::StaAbs => self.store_reg(AddrModes::Absolute, Registers::A),
            Ops::StaAbsX => self.store_reg(AddrModes::AbsoluteX, Registers::A),
            Ops::StaAbsY => self.store_reg(AddrModes::AbsoluteY, Registers::A),
            Ops::StaIndX => self.store_reg(AddrModes::IndirectX, Registers::A),
            Ops::StaIndY => self.store_reg(AddrModes::IndirectY, Registers::A),

            Ops::StxZp => self.store_reg(AddrModes::ZeroPage, Registers::X),
            Ops::StxZpY => self.store_reg(AddrModes::ZeroPageY, Registers::X),
            Ops::StxAbs => self.store_reg(AddrModes::Absolute, Registers::X),

            Ops::StyZp => self.store_reg(AddrModes::ZeroPage, Registers::Y),
            Ops::StyZpX => self.store_reg(AddrModes::ZeroPageX, Registers::Y),
            Ops::StyAbs => self.store_reg(AddrModes::Absolute, Registers::Y),

            Ops::Tax => self.transfer_reg(TransferOption::A, TransferOption::X),
            Ops::Tay => self.transfer_reg(TransferOption::A, TransferOption::Y),
            Ops::Tsx => self.transfer_reg(TransferOption::S, TransferOption::X),
            Ops::Txa => self.transfer_reg(TransferOption::X, TransferOption::A),
            Ops::Txs => self.transfer_reg(TransferOption::X, TransferOption::S),
            Ops::Tya => self.transfer_reg(TransferOption::Y, TransferOption::A),

            _ => self.cry(op as u8)
        }
    }

    fn cry(&self, op: u8) {
        panic!("Invalid opcode given: {:#02x}", op);
    }

    fn get_address(&mut self, mode: AddrModes) -> usize {
        match mode {
            AddrModes::ZeroPage => self.fetch_zp(),
            AddrModes::ZeroPageX => self.fetch_zpx(),
            AddrModes::Absolute => self.fetch_abs(),
            AddrModes::AbsoluteX => self.fetch_absx(),
            AddrModes::AbsoluteY => self.fetch_absy(),
            AddrModes::IndirectX => self.fetch_indirectx(),
            AddrModes::IndirectY => self.fetch_indirecty(),
            AddrModes::Indirect => self.fetch_indirect(),
            _=> panic!("Invalid AddrMode for fetching address: {:?}", mode),
        }
    }

    fn get_value(&mut self, mode: AddrModes) -> u8 {
        match mode {
            AddrModes::Immediate => self.fetch_next_byte(),
            _ => {
                let addr = self.get_address(mode);
                self.memory[addr]
            }
        }
    }

    fn transfer_reg(&mut self, from: TransferOption, to: TransferOption) {
        let val = match from {
            TransferOption::A => self.rega,
            TransferOption::X => self.regx,
            TransferOption::Y => self.regy,
            TransferOption::S => self.sp as u8,
        };

        match to {
            TransferOption::A => self.rega = val,
            TransferOption::X => self.regx = val,
            TransferOption::Y => self.regy = val,
            TransferOption::S => self.sp = val as usize,
        };
        if to != TransferOption::S {
            self.flags.zero = val == 0;
            self.flags.negative = get_top_bit(val);
        }
    }

    fn store_reg(&mut self, mode: AddrModes, reg: Registers) {
        let addr = self.get_address(mode);
        self.memory[addr] = match reg {
            Registers::A => self.rega,
            Registers::X => self.regx,
            Registers::Y => self.regy
        };
    }

    fn sbc(&mut self, mode: AddrModes) {
        let val = self.get_value(mode);
        let (res, carry) = self.rega.overflowing_sub(val + (!self.flags.carry as u8));
        self.flags.overflow = is_overflow(res, self.rega, val);
        self.rega = res;
        self.flags.carry = carry;
        self.flags.zero = self.rega == 0;
        self.flags.negative = get_top_bit(self.rega);
    }

    fn rti(&mut self) {
        self.pull_register(false);
        self.pull_pc();
    }

    fn ror_or_rol(&mut self, mode: AddrModes, is_ror: bool) {
        let old_carry = self.flags.carry;
        let (val, carry) = match mode {
            AddrModes::Accumulator => {
                let mut carry = false;

                if !is_ror {
                    carry = get_top_bit(self.rega);
                    self.rega <<= 1;
                    if old_carry {
                        self.rega |= 0x1;
                    }
                } else {
                    carry = check_bit(self.rega, 1);
                    self.rega >>= 1;
                    if old_carry {
                        self.rega |= 0x80;
                    }
                }
                (self.rega, carry)
            },
            AddrModes::ZeroPage | AddrModes::ZeroPageX | AddrModes::Absolute | AddrModes::AbsoluteX => {
                let addr = self.get_address(mode);
                let mut carry = false;
                if !is_ror {
                    carry = get_top_bit(self.memory[addr]);
                    self.memory[addr] <<= 1;
                    if old_carry {
                        self.memory[addr] &= 0x1;
                    }
                } else {
                    carry = check_bit(self.memory[addr], 1);
                    self.memory[addr] >>= 1;
                    if old_carry {
                        self.memory[addr] &= 0x80;
                    }
                }
                (self.memory[addr], carry)
            },
            _ => panic!("Invalid AddrMode for ROL: {:?}", mode),
        };

        self.flags.zero = val == 0;
        self.flags.negative = get_top_bit(val);
        self.flags.carry = carry;
    }

    fn pull_register(&mut self, pull_accum: bool) {
        let popped = self.pop_stack();
        match pull_accum {
            true =>{
                self.rega = popped;
                self.flags.zero = self.rega == 0;
                self.flags.negative = get_top_bit(self.rega);
            },
            false => {
                self.flags = Flags::from(popped);
            }
        };

    }

    fn or(&mut self, mode: AddrModes) {
        let val = self.get_value(mode);
        self.rega |= val;
        self.flags.zero = self.rega == 0;
        self.flags.negative = get_top_bit(self.rega);
    }

    fn lsr(&mut self, mode: AddrModes) {
        let (val, carry) = match mode {
            AddrModes::Accumulator => {
                let carry = check_bit(self.rega, 1);
                self.rega >>= 1;
                (self.rega, carry)
            },
            AddrModes::ZeroPage | AddrModes::ZeroPageX | AddrModes::Absolute | AddrModes::AbsoluteX => {
                let addr = self.get_address(mode);
                let carry = check_bit(self.memory[addr], 1);
                self.memory[addr] >>= 1;
                (self.memory[addr], carry)
            },
            _ => panic!("Invalid AddrMode for LSR: {:?}", mode),
        };

        self.flags.zero = val == 0;
        self.flags.negative = get_top_bit(val);
        self.flags.carry = carry;
    }

    fn ld_reg(&mut self, mode: AddrModes, reg: Registers) {
        let val = self.get_value(mode);
        match reg {
            Registers::A => self.rega = val,
            Registers::X => self.regx = val,
            Registers::Y => self.regy = val,
        };

        self.flags.zero = val == 0;
        self.flags.negative = get_top_bit(val);
    }

    fn jsr(&mut self) {
        self.save_pc(true);
        self.pc = self.get_address(AddrModes::Absolute) as u16;
    }

    fn jmp(&mut self, mode: AddrModes) {
        self.pc = self.get_address(mode) as u16;
    }

    fn inc_reg(&mut self, reg: Registers) {
        let affected = match reg {
            Registers::X => {
                self.regx += 1;
                self.regx
            },
            Registers::Y =>  {
                self.regy += 1;
                self.regy
            },
            Registers::A => panic!("Invalid decrement register A"),
        };
        self.flags.zero = affected == 0;
        self.flags.negative = get_top_bit(affected);
    }

    fn inc(&mut self, mode: AddrModes) {
        let addr = self.get_address(mode);
        self.memory[addr] += 1;
        self.flags.zero = self.memory[addr] == 0;
        self.flags.negative = get_top_bit(self.memory[addr]);
    }

    fn eor(&mut self, mode: AddrModes) {
        let value = self.get_value(mode);
        self.rega ^= value;
        self.flags.zero = self.rega == 0;
        self.flags.negative = get_top_bit(self.rega);
    }

    fn dec_reg(&mut self, reg: Registers) {
        let affected = match reg {
            Registers::X => {
                self.regx -= 1;
                self.regx
            },
            Registers::Y =>  {
                self.regy -= 1;
                self.regy
            },
            Registers::A => panic!("Invalid decrement register A"),
        };
        self.flags.zero = affected == 0;
        self.flags.negative = get_top_bit(affected);
    }

    fn dec(&mut self, mode: AddrModes) {
        let addr = self.get_address(mode);
        self.memory[addr] -= 1;
        self.flags.zero = self.memory[addr] == 0;
        self.flags.negative = get_top_bit(self.memory[addr]);
    }

    fn cmp(&mut self, mode: AddrModes, register: Registers) {
        let val = self.get_value(mode);
        let to_compare = match register {
            Registers::A => self.rega,
            Registers::X => self.regx,
            Registers::Y => self.regy
        };
        self.flags.set_for_cmp(to_compare, val);
    }

    fn bvs_or_bvc(&mut self, check_for_set: bool) {
        let displace: u16 = self.fetch_next_byte().into();

        if self.flags.overflow == check_for_set {
            self.pc += displace;
        }
    }

    fn brk(&mut self) {
        self.save_pc(false);
        self.save_status();
        let (lower, upper) = (self.memory[0xFFFE], self.memory[0xFFFF]);
        self.pc = combine_bytes(upper.into(), lower.into());
        self.flags.set_breaks();
    }

    fn bpl(&mut self) {
        let displace: u16 = self.fetch_next_byte().into();

        if !self.flags.negative {
            self.pc += displace;
        }
    }

    fn bmi(&mut self) {
        let displace: u16 = self.fetch_next_byte().into();

        if self.flags.negative {
            self.pc += displace;
        }
    }

    fn bit(&mut self, mode: AddrModes) {
        let val = self.get_value(mode);
        let res = self.rega & val;
        self.flags.zero = res == 0;
        self.flags.overflow = check_bit(val, 6);
        self.flags.negative = check_bit(val, 7);
    }

    fn beq_or_bne(&mut self, should_be_zero: bool) {
        let displace: u16 = self.fetch_next_byte().into();

        if self.flags.zero == should_be_zero {
            self.pc += displace;
        }
    }


    fn bcc_or_bcs(&mut self, branch_on_set: bool) {
        // Might as well do this before since the pc needs to be incremented anyway
        let displace: u16 = self.fetch_next_byte().into();

        if self.flags.carry == branch_on_set {
            self.pc += displace;
        }
    }

    fn asl(&mut self, mode: AddrModes) {
        let (val, carry) = match mode {
            AddrModes::Accumulator => {
                let carry = get_top_bit(self.rega);
                self.rega <<= 1;
                (self.rega, carry)
            },
            AddrModes::ZeroPage | AddrModes::ZeroPageX | AddrModes::Absolute | AddrModes::AbsoluteX => {
                let addr = self.get_address(mode);
                let carry = get_top_bit(self.memory[addr]);
                self.memory[addr] <<= 1;
                (self.memory[addr], carry)
            },
            _ => panic!("Invalid AddrMode for ASL: {:?}", mode),
        };

        self.flags.zero = val == 0;
        self.flags.negative = get_top_bit(val);
        self.flags.carry = carry;
    }

    fn and(&mut self, mode: AddrModes) {
        let val = self.get_value(mode);
        self.rega &= val;
        self.flags.zero = self.rega == 0;
        self.flags.negative = get_top_bit(self.rega);
    }

    fn adc(&mut self, mode: AddrModes) {
        let val = self.get_value(mode);
        let (res, carry) = self.rega.overflowing_add(val + (self.flags.carry as u8));
        self.flags.overflow = is_overflow(res, self.rega, val);
        self.rega = res;
        self.flags.carry = carry;
        self.flags.zero = self.rega == 0;
        self.flags.negative = get_top_bit(self.rega);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tya() {
        let mut cpu = Cpu::new();
        cpu.regy = 14;
        cpu.exec_instruction(Ops::Tya);
        assert_eq!(cpu.rega, 14);
    }

    #[test]
    fn test_txs() {
        let mut cpu = Cpu::new();
        cpu.regx = 14;
        cpu.exec_instruction(Ops::Txs);
        assert_eq!(cpu.sp, 14);
    }

    #[test]
    fn test_txa() {
        let mut cpu = Cpu::new();
        cpu.regx = 14;
        cpu.exec_instruction(Ops::Txa);
        assert_eq!(cpu.rega, 14);
    }

    #[test]
    fn test_tsx() {
        let mut cpu = Cpu::new();
        cpu.sp = 14;
        cpu.exec_instruction(Ops::Tsx);
        assert_eq!(cpu.regx, 14);
    }

    #[test]
    fn test_tay() {
        let mut cpu = Cpu::new();
        cpu.rega = 14;
        cpu.exec_instruction(Ops::Tay);
        assert_eq!(cpu.regy, 14);
    }

    #[test]
    fn test_tax() {
        let mut cpu = Cpu::new();
        cpu.rega = 14;
        cpu.exec_instruction(Ops::Tax);
        assert_eq!(cpu.regx, 14);
    }

    #[test]
    fn test_stx() {
        let mut cpu = Cpu::new();
        cpu.memory[1] = 0x56;
        cpu.memory[2] = 0x63;
        cpu.regx = 45;
        cpu.exec_instruction(Ops::StxAbs);
        assert_eq!(cpu.memory[0x5663], 45);
    }

    #[test]
    fn test_sty() {
        let mut cpu = Cpu::new();
        cpu.memory[1] = 0x56;
        cpu.memory[2] = 0x63;
        cpu.regy = 45;
        cpu.exec_instruction(Ops::StyAbs);
        assert_eq!(cpu.memory[0x5663], 45);
    }

    #[test]
    fn test_sta() {
        let mut cpu = Cpu::new();
        cpu.memory[1] = 0x56;
        cpu.memory[2] = 0x63;
        cpu.rega = 45;
        cpu.exec_instruction(Ops::StaAbs);
        assert_eq!(cpu.memory[0x5663], 45);
    }

    #[test]
    fn test_sbczp_carry() {
        let mut cpu = Cpu::new();
        let loc: usize = 0x64;
        cpu.memory[1] = loc as u8;
        cpu.memory[loc] = 10;
        cpu.rega = 30;
        cpu.flags.carry = true;
        cpu.exec_instruction(Ops::SbcZp);
        assert_eq!(cpu.rega, 20);
    }

    #[test]
    fn test_sbczp_no_carry() {
        let mut cpu = Cpu::new();
        let loc: usize = 0x64;
        cpu.memory[1] = loc as u8;
        cpu.memory[loc] = 10;
        cpu.rega = 30;
        cpu.exec_instruction(Ops::SbcZp);
        assert_eq!(cpu.rega, 19);
    }

    #[test]
    fn test_rti() {
        let mut cpu = Cpu::new();
        cpu.flags.carry = true;
        cpu.pc = 0x4567;
        cpu.save_pc(false);
        cpu.save_status();
        cpu.flags.carry = false;
        cpu.pc = 0x4598;
        cpu.exec_instruction(Ops::Rti);
        assert_eq!(cpu.pc, 0x4567);
        assert!(cpu.flags.carry);
    }

    #[test]
    fn test_ror_accum() {
        let mut cpu = Cpu::new();
        cpu.rega = 0x1;
        cpu.flags.carry = true;
        cpu.exec_instruction(Ops::RorAccum);
        assert_eq!(cpu.rega, 0x80);
        assert!(!cpu.flags.zero);
        assert!(cpu.flags.negative);
        assert!(cpu.flags.carry);
    }

    #[test]
    fn test_rol_accum() {
        let mut cpu = Cpu::new();
        cpu.rega = 0x80;
        cpu.flags.carry = true;
        cpu.exec_instruction(Ops::RolAccum);
        assert_eq!(cpu.rega, 1);
        assert!(!cpu.flags.zero);
        assert!(!cpu.flags.negative);
        assert!(cpu.flags.carry);
    }

    #[test]
    fn test_pla() {
        let mut cpu = Cpu::new();
        cpu.push_stack(0x42);
        cpu.exec_instruction(Ops::Pla);
        assert_eq!(cpu.rega, 0x42);
    }

    #[test]
    fn test_plp() {
        let mut cpu = Cpu::new();
        cpu.push_stack(0x03);
        cpu.exec_instruction(Ops::Plp);
        assert!(cpu.flags.carry);
        assert!(cpu.flags.zero);
    }

    #[test]
    fn test_php() {
        let mut cpu = Cpu::new();
        cpu.flags.carry = true;
        cpu.exec_instruction(Ops::Php);
        assert_eq!(cpu.peek_stack(), 1);
    }

    #[test]
    fn test_pha() {
        let mut cpu = Cpu::new();
        cpu.rega = 10;
        cpu.exec_instruction(Ops::Pha);
        assert_eq!(cpu.peek_stack(), 10);
    }

    #[test]
    fn test_or_zero() {
        let mut cpu = Cpu::new();
        cpu.rega = 0x0;
        cpu.memory[1] = 0x0;
        cpu.exec_instruction(Ops::ORI);
        assert_eq!(cpu.rega, 0);
        assert!(!cpu.flags.negative);
        assert!(cpu.flags.zero);
    }

    #[test]
    fn test_or() {
        let mut cpu = Cpu::new();
        cpu.rega = 0xFF;
        cpu.memory[1] = 0xFF;
        cpu.exec_instruction(Ops::ORI);
        assert_eq!(cpu.rega, 0xFF);
        assert!(cpu.flags.negative);
        assert!(!cpu.flags.zero);
    }

    #[test]
    fn test_nop() {
        let mut cpu = Cpu::new();
        let old_pc = cpu.pc;
        cpu.exec_instruction(Ops::Nop);
        assert_eq!(old_pc, cpu.pc);
    }

    #[test]
    fn test_lsr_accum_negative() {
        let mut cpu = Cpu::new();
        cpu.rega = 0x71;
        cpu.exec_instruction(Ops::LsrAccum);
        assert_eq!(cpu.rega, 0x71 >> 1);
        assert!(!cpu.flags.zero);
        assert!(!cpu.flags.negative);
        assert!(cpu.flags.carry);
    }

    #[test]
    fn test_lsr_accum_zero_carry() {
        let mut cpu = Cpu::new();
        cpu.rega = 0x1;
        cpu.exec_instruction(Ops::LsrAccum);
        assert_eq!(cpu.rega, 0);
        assert!(cpu.flags.zero);
        assert!(!cpu.flags.negative);
        assert!(cpu.flags.carry);
    }

    #[test]
    fn test_lsr_accum() {
        let mut cpu = Cpu::new();
        cpu.rega = 2;
        cpu.exec_instruction(Ops::LsrAccum);
        assert_eq!(cpu.rega, 1);
        assert!(!cpu.flags.zero);
        assert!(!cpu.flags.negative);
        assert!(!cpu.flags.carry);
    }

    #[test]
    fn test_ldy() {
        let mut cpu = Cpu::new();
        cpu.memory[1] = 45;
        cpu.exec_instruction(Ops::LdyI);
        assert_eq!(cpu.regy, 45);
        assert!(!cpu.flags.zero)
    }

    #[test]
    fn test_ldx() {
        let mut cpu = Cpu::new();
        cpu.memory[1] = 45;
        cpu.exec_instruction(Ops::LdxI);
        assert_eq!(cpu.regx, 45);
        assert!(!cpu.flags.zero)
    }

    #[test]
    fn test_lda() {
        let mut cpu = Cpu::new();
        cpu.memory[1] = 45;
        cpu.exec_instruction(Ops::LdaI);
        assert_eq!(cpu.rega, 45);
        assert!(!cpu.flags.zero)
    }

    #[test]
    fn test_jsr() {
        let mut cpu = Cpu::new();
        cpu.memory[0x2344] = 0xFF;
        cpu.memory[0x2345] = 0xFF;
        cpu.pc = 0x2343;
        cpu.exec_instruction(Ops::Jsr);
        assert_eq!(cpu.peek_stack(), 0x23);
        assert_eq!(cpu.memory[(cpu.sp + 2) as usize], 0x42);
        assert_eq!(cpu.pc, 0xFFFF);
    }

    #[test]
    fn test_jump_ind() {
        let mut cpu = Cpu::new();
        cpu.memory[0x0120] = 0xFC;
        cpu.memory[0x0121] = 0xBA;
        cpu.memory[1] = 0x01;
        cpu.memory[2] = 0x20;
        cpu.exec_instruction(Ops::JmpInd);
        assert_eq!(cpu.pc, 0xBAFC);
    }

    #[test]
    fn test_incy() {
        let mut cpu = Cpu::new();
        cpu.regy = 12;
        cpu.exec_instruction(Ops::IncY);
        assert_eq!(cpu.regy, 13);
        assert!(!cpu.flags.zero);
        assert!(!cpu.flags.negative);
    }

    #[test]
    fn test_incx() {
        let mut cpu = Cpu::new();
        cpu.regx = 12;
        cpu.exec_instruction(Ops::IncX);
        assert_eq!(cpu.regx, 13);
        assert!(!cpu.flags.zero);
        assert!(!cpu.flags.negative);
    }

    #[test]
    fn test_inc() {
        let mut cpu = Cpu::new();
        cpu.memory[1] = 10;
        cpu.memory[10] = 12;
        cpu.exec_instruction(Ops::IncZp);
        assert_eq!(cpu.memory[10], 13);
        assert!(!cpu.flags.zero);
        assert!(!cpu.flags.negative);
    }

    #[test]
    fn test_eor_zero() {
        let mut cpu = Cpu::new();
        cpu.rega = 0xFF;
        cpu.memory[1] = 0xFF;
        cpu.exec_instruction(Ops::EORI);
        assert_eq!(cpu.rega, 0);
        assert!(!cpu.flags.negative);
        assert!(cpu.flags.zero);
    }

    #[test]
    fn test_eor() {
        let mut cpu = Cpu::new();
        cpu.rega = 0x0F;
        cpu.memory[1] = 0xF0;
        cpu.exec_instruction(Ops::EORI);
        assert_eq!(cpu.rega, 0xFF);
        assert!(cpu.flags.negative);
        assert!(!cpu.flags.zero);
    }

    #[test]
    fn test_decy() {
        let mut cpu = Cpu::new();
        cpu.regy = 12;
        cpu.exec_instruction(Ops::DecY);
        assert_eq!(cpu.regy, 11);
        assert!(!cpu.flags.zero);
        assert!(!cpu.flags.negative);
    }

    #[test]
    fn test_decx_zero() {
        let mut cpu = Cpu::new();
        cpu.regx = 1;
        cpu.exec_instruction(Ops::DecX);
        assert_eq!(cpu.regx, 0);
        assert!(cpu.flags.zero);
        assert!(!cpu.flags.negative);
    }

    #[test]
    fn test_decx() {
        let mut cpu = Cpu::new();
        cpu.regx = 12;
        cpu.exec_instruction(Ops::DecX);
        assert_eq!(cpu.regx, 11);
        assert!(!cpu.flags.zero);
        assert!(!cpu.flags.negative);
    }

    #[test]
    fn test_dec_zero() {
        let mut cpu = Cpu::new();
        cpu.memory[1] = 10;
        cpu.memory[10] = 1;
        cpu.exec_instruction(Ops::DecZp);
        assert_eq!(cpu.memory[10], 0);
        assert!(cpu.flags.zero);
        assert!(!cpu.flags.negative);
    }

    #[test]
    fn test_dec() {
        let mut cpu = Cpu::new();
        cpu.memory[1] = 10;
        cpu.memory[10] = 12;
        cpu.exec_instruction(Ops::DecZp);
        assert_eq!(cpu.memory[10], 11);
        assert!(!cpu.flags.zero);
        assert!(!cpu.flags.negative);
    }

    #[test]
    fn test_cmpy_carry() {
        let mut cpu = Cpu::new();
        cpu.regy = 12;
        cpu.memory[1] = 10;
        cpu.exec_instruction(Ops::CmpYI);
        assert!(cpu.flags.carry);
        assert!(!cpu.flags.zero);
        assert!(!cpu.flags.negative);
    }

    #[test]
    fn test_cmpy_neg() {
        let mut cpu = Cpu::new();
        cpu.regy = 10;
        cpu.memory[1] = 12;
        cpu.exec_instruction(Ops::CmpYI);
        assert!(!cpu.flags.carry);
        assert!(!cpu.flags.zero);
        assert!(cpu.flags.negative);
    }

    #[test]
    fn test_cmpy_eq() {
        let mut cpu = Cpu::new();
        cpu.regy = 10;
        cpu.memory[1] = 10;
        cpu.exec_instruction(Ops::CmpYI);
        assert!(cpu.flags.carry);
        assert!(cpu.flags.zero);
        assert!(!cpu.flags.negative);
    }

    #[test]
    fn test_cmpx_carry() {
        let mut cpu = Cpu::new();
        cpu.regx = 12;
        cpu.memory[1] = 10;
        cpu.exec_instruction(Ops::CmpXI);
        assert!(cpu.flags.carry);
        assert!(!cpu.flags.zero);
        assert!(!cpu.flags.negative);
    }

    #[test]
    fn test_cmpx_neg() {
        let mut cpu = Cpu::new();
        cpu.regx = 10;
        cpu.memory[1] = 12;
        cpu.exec_instruction(Ops::CmpXI);
        assert!(!cpu.flags.carry);
        assert!(!cpu.flags.zero);
        assert!(cpu.flags.negative);
    }

    #[test]
    fn test_cmpx_eq() {
        let mut cpu = Cpu::new();
        cpu.regx = 10;
        cpu.memory[1] = 10;
        cpu.exec_instruction(Ops::CmpXI);
        assert!(cpu.flags.carry);
        assert!(cpu.flags.zero);
        assert!(!cpu.flags.negative);
    }

    #[test]
    fn test_cmp_carry() {
        let mut cpu = Cpu::new();
        cpu.rega = 12;
        cpu.memory[1] = 10;
        cpu.exec_instruction(Ops::CmpI);
        assert!(cpu.flags.carry);
        assert!(!cpu.flags.zero);
        assert!(!cpu.flags.negative);
    }

    #[test]
    fn test_cmp_neg() {
        let mut cpu = Cpu::new();
        cpu.rega = 10;
        cpu.memory[1] = 12;
        cpu.exec_instruction(Ops::CmpI);
        assert!(!cpu.flags.carry);
        assert!(!cpu.flags.zero);
        assert!(cpu.flags.negative);
    }

    #[test]
    fn test_cmp_eq() {
        let mut cpu = Cpu::new();
        cpu.rega = 10;
        cpu.memory[1] = 10;
        cpu.exec_instruction(Ops::CmpI);
        assert!(cpu.flags.carry);
        assert!(cpu.flags.zero);
        assert!(!cpu.flags.negative);
    }

    #[test]
    fn test_clear_overflow() {
        let mut cpu = Cpu::new();
        cpu.flags.overflow = true;
        cpu.exec_instruction(Ops::Clv);
        assert!(!cpu.flags.overflow);
    }

    #[test]
    fn test_clear_interrupt() {
        let mut cpu = Cpu::new();
        cpu.flags.inter_disable = true;
        cpu.exec_instruction(Ops::Cli);
        assert!(!cpu.flags.inter_disable);
    }

    #[test]
    fn test_clear_decimal() {
        let mut cpu = Cpu::new();
        cpu.flags.decimal = true;
        cpu.exec_instruction(Ops::Cld);
        assert!(!cpu.flags.decimal);
    }

    #[test]
    fn test_clear_carry() {
        let mut cpu = Cpu::new();
        cpu.flags.carry = true;
        cpu.exec_instruction(Ops::Clc);
        assert!(!cpu.flags.carry);
    }

    #[test]
    fn test_bvs_false() {
        let mut cpu = Cpu::new();
        cpu.flags.overflow = false;
        let old_pc = cpu.pc;
        cpu.memory[1] = 10;
        cpu.exec_instruction(Ops::Bvs);
        assert_eq!(cpu.pc, old_pc + 1);
    }

    #[test]
    fn test_bvs_true() {
        let mut cpu = Cpu::new();
        cpu.flags.overflow = true;
        let old_pc = cpu.pc;
        cpu.memory[1] = 10;
        cpu.exec_instruction(Ops::Bvs);
        assert_eq!(cpu.pc, old_pc + 11);
    }

    #[test]
    fn test_bvc_false() {
        let mut cpu = Cpu::new();
        cpu.flags.overflow = true;
        let old_pc = cpu.pc;
        cpu.memory[1] = 10;
        cpu.exec_instruction(Ops::Bvc);
        assert_eq!(cpu.pc, old_pc + 1);
    }

    #[test]
    fn test_bvc_true() {
        let mut cpu = Cpu::new();
        cpu.flags.overflow = false;
        let old_pc = cpu.pc;
        cpu.memory[1] = 10;
        cpu.exec_instruction(Ops::Bvc);
        assert_eq!(cpu.pc, old_pc + 11);
    }

    #[test]
    fn test_brk() {
        let mut cpu = Cpu::new();
        cpu.memory[0xFFFE] = 0x98;
        cpu.memory[0xFFFF] = 0x45;
        cpu.pc = 0x3456;
        cpu.flags.carry = true;
        cpu.exec_instruction(Ops::Brk);
        assert_eq!(cpu.peek_stack(), 1);
        assert_eq!(cpu.memory[(cpu.sp + 2) as usize], 0x34);
        assert_eq!(cpu.memory[(cpu.sp + 3) as usize], 0x56);
        assert_eq!(cpu.pc, 0x4598);
        assert!(cpu.flags.break1 && cpu.flags.break2);
    }

    #[test]
    fn test_bpl_false() {
        let mut cpu = Cpu::new();
        cpu.flags.negative = true;
        let old_pc = cpu.pc;
        cpu.memory[1] = 10;
        cpu.exec_instruction(Ops::Bpl);
        assert_eq!(cpu.pc, old_pc + 1);
    }

    #[test]
    fn test_bpl_true() {
        let mut cpu = Cpu::new();
        cpu.flags.negative = false;
        let old_pc = cpu.pc;
        cpu.memory[1] = 10;
        cpu.exec_instruction(Ops::Bpl);
        assert_eq!(cpu.pc, old_pc + 11);
    }

    #[test]
    fn test_bne_false() {
        let mut cpu = Cpu::new();
        cpu.flags.zero = true;
        let old_pc = cpu.pc;
        cpu.memory[1] = 10;
        cpu.exec_instruction(Ops::Bne);
        assert_eq!(cpu.pc, old_pc + 1);
    }

    #[test]
    fn test_bne_true() {
        let mut cpu = Cpu::new();
        cpu.flags.zero = false;
        let old_pc = cpu.pc;
        cpu.memory[1] = 10;
        cpu.exec_instruction(Ops::Bne);
        assert_eq!(cpu.pc, old_pc + 11);
    }

    #[test]
    fn test_bmi_false() {
        let mut cpu = Cpu::new();
        cpu.flags.negative = false;
        let old_pc = cpu.pc;
        cpu.memory[1] = 10;
        cpu.exec_instruction(Ops::Bmi);
        assert_eq!(cpu.pc, old_pc + 1);
    }

    #[test]
    fn test_bmi_true() {
        let mut cpu = Cpu::new();
        cpu.flags.negative = true;
        let old_pc = cpu.pc;
        cpu.memory[1] = 10;
        cpu.exec_instruction(Ops::Bmi);
        assert_eq!(cpu.pc, old_pc + 11);
    }

    #[test]
    fn test_bit() {
        let mut cpu = Cpu::new();
        cpu.memory[1] = 10;
        cpu.memory[10] = 0x01;
        cpu.rega = 0x0F;
        cpu.exec_instruction(Ops::BitZp);
        assert!(!cpu.flags.zero);
        assert!(!cpu.flags.overflow);
        assert!(!cpu.flags.negative);
    }

    #[test]
    fn test_bit_zero() {
        let mut cpu = Cpu::new();
        cpu.memory[1] = 10;
        cpu.memory[10] = 0xF0;
        cpu.rega = 0x0F;
        cpu.exec_instruction(Ops::BitZp);
        assert!(cpu.flags.zero);
        assert!(cpu.flags.overflow);
        assert!(cpu.flags.negative);
    }

    #[test]
    fn test_beq_false() {
        let mut cpu = Cpu::new();
        cpu.flags.zero = false;
        let old_pc = cpu.pc;
        cpu.memory[1] = 10;
        cpu.exec_instruction(Ops::Beq);
        assert_eq!(cpu.pc, old_pc + 1);
    }

    #[test]
    fn test_beq_true() {
        let mut cpu = Cpu::new();
        cpu.flags.zero = true;
        let old_pc = cpu.pc;
        cpu.memory[1] = 10;
        cpu.exec_instruction(Ops::Beq);
        assert_eq!(cpu.pc, old_pc + 11);
    }

    #[test]
    fn test_bcs_false() {
        let mut cpu = Cpu::new();
        cpu.flags.carry = false;
        let old_pc = cpu.pc;
        cpu.memory[1] = 10;
        cpu.exec_instruction(Ops::Bcs);
        assert_eq!(cpu.pc, old_pc + 1);
    }

    #[test]
    fn test_bcs_true() {
        let mut cpu = Cpu::new();
        cpu.flags.carry = true;
        let old_pc = cpu.pc;
        cpu.memory[1] = 10;
        cpu.exec_instruction(Ops::Bcs);
        assert_eq!(cpu.pc, old_pc + 11);
    }

    #[test]
    fn test_bcc_false() {
        let mut cpu = Cpu::new();
        cpu.flags.carry = true;
        let old_pc = cpu.pc;
        cpu.memory[1] = 10;
        cpu.exec_instruction(Ops::Bcc);
        assert_eq!(cpu.pc, old_pc + 1);
    }

    #[test]
    fn test_bcc_true() {
        let mut cpu = Cpu::new();
        cpu.flags.carry = false;
        let old_pc = cpu.pc;
        cpu.memory[1] = 10;
        cpu.exec_instruction(Ops::Bcc);
        assert_eq!(cpu.pc, old_pc + 11);
    }

    #[test]
    fn test_asl_accum_negative() {
        let mut cpu = Cpu::new();
        cpu.rega = 0x71;
        cpu.exec_instruction(Ops::AslAccum);
        assert_eq!(cpu.rega, 0x71 << 1);
        assert!(!cpu.flags.zero);
        assert!(cpu.flags.negative);
        assert!(!cpu.flags.carry);
    }

    #[test]
    fn test_asl_accum_zero_carry() {
        let mut cpu = Cpu::new();
        cpu.rega = 0x80;
        cpu.exec_instruction(Ops::AslAccum);
        assert_eq!(cpu.rega, 0);
        assert!(cpu.flags.zero);
        assert!(!cpu.flags.negative);
        assert!(cpu.flags.carry);
    }

    #[test]
    fn test_asl_accum() {
        let mut cpu = Cpu::new();
        cpu.rega = 2;
        cpu.exec_instruction(Ops::AslAccum);
        assert_eq!(cpu.rega, 4);
        assert!(!cpu.flags.zero);
        assert!(!cpu.flags.negative);
        assert!(!cpu.flags.carry);
    }

    #[test]
    fn test_and_negative() {
        let mut cpu = Cpu::new();
        cpu.memory[1] = 0xFF;
        cpu.rega = 0x89;
        cpu.exec_instruction(Ops::AndI);
        assert_eq!(cpu.rega, 0xFF & 0x89);
        assert!(cpu.flags.negative);
    }

    #[test]
    fn test_and_zero() {
        let mut cpu = Cpu::new();
        cpu.memory[1] = 20;
        cpu.exec_instruction(Ops::AndI);
        assert_eq!(cpu.rega, 0);
        assert!(cpu.flags.zero);
    }

    #[test]
    fn test_and() {
        let mut cpu = Cpu::new();
        cpu.memory[1] = 0x0F;
        cpu.rega = 0x09;
        cpu.exec_instruction(Ops::AndI);
        assert_eq!(cpu.rega, 0x0F & 0x09);
        assert!(!cpu.flags.zero);
        assert!(!cpu.flags.negative);
    }

    #[test]
    fn test_adci_overflow_true() {
        let mut cpu = Cpu::new();
        cpu.memory[1] = 80;
        cpu.rega = 80;
        cpu.exec_instruction(Ops::AdcI);
        assert_eq!(cpu.rega, 160);
        assert!(cpu.flags.overflow);
    }

    #[test]
    fn test_adci_negative_true() {
        let mut cpu = Cpu::new();
        cpu.memory[1] = 0xFF;
        cpu.exec_instruction(Ops::AdcI);
        assert_eq!(cpu.rega, 0xFF);
        assert!(cpu.flags.negative);
    }

    #[test]
    fn test_adcindy() {
        let mut cpu = Cpu::new();
        cpu.memory[0x14] = 0x24;
        cpu.memory[0x24] = 0x45;
        cpu.memory[0x25] = 0x34;
        cpu.memory[1] = 0x14 - 10;
        cpu.regy = 10;
        cpu.memory[0x3445] = 100;
        cpu.exec_instruction(Ops::AdcIndY);
        assert_eq!(cpu.rega, 100);
        assert!(!cpu.flags.overflow);
    }

    #[test]
    fn test_adcindx() {
        let mut cpu = Cpu::new();
        cpu.memory[0x14] = 0x24;
        cpu.memory[0x24] = 0x45;
        cpu.memory[0x25] = 0x34;
        cpu.memory[1] = 0x14 - 10;
        cpu.regx = 10;
        cpu.memory[0x3445] = 100;
        cpu.exec_instruction(Ops::AdcIndX);
        assert_eq!(cpu.rega, 100);
    }

    #[test]
    fn test_adcabsy() {
        let mut cpu = Cpu::new();
        cpu.memory[1] = 0x11;
        cpu.memory[2] = 0x11;
        cpu.memory[0x1111 + 10] = 23;
        cpu.regy = 10;
        cpu.exec_instruction(Ops::AdcAbsY);
        assert_eq!(cpu.rega, 23);
    }

    #[test]
    fn test_adcabsx() {
        let mut cpu = Cpu::new();
        cpu.memory[1] = 0x11;
        cpu.memory[2] = 0x11;
        cpu.memory[0x1111 + 10] = 23;
        cpu.regx = 10;
        cpu.exec_instruction(Ops::AdcAbsX);
        assert_eq!(cpu.rega, 23);
    }

    #[test]
    fn test_adcabs() {
        let mut cpu = Cpu::new();
        cpu.memory[1] = 0x11;
        cpu.memory[2] = 0x11;
        cpu.memory[0x1111] = 23;
        cpu.exec_instruction(Ops::AdcAbs);
        assert_eq!(cpu.rega, 23);
    }

    #[test]
    fn test_adczpx() {
        let mut cpu = Cpu::new();
        let loc: usize = 0x64;
        cpu.memory[1] = loc as u8;
        cpu.memory[loc+10] = 23;
        cpu.regx = 10;
        cpu.exec_instruction(Ops::AdcZpX);
        assert_eq!(cpu.rega, 23);
    }

    #[test]
    fn test_adci_no_over() {
        let mut cpu = Cpu::new();
        cpu.memory[1] = 23;
        cpu.exec_instruction(Ops::AdcI);
        assert_eq!(cpu.rega, 23);
        assert!(!cpu.flags.negative);
    }

    #[test]
    fn test_adci_over() {
        let mut cpu = Cpu::new();
        cpu.memory[1] = 5;
        cpu.rega = 255;
        cpu.exec_instruction(Ops::AdcI);
        assert_eq!(cpu.rega, 4);
        assert!(cpu.flags.carry);
    }

    #[test]
    fn test_adci_zero() {
        let mut cpu = Cpu::new();
        cpu.exec_instruction(Ops::AdcI);
        assert_eq!(cpu.rega, 0);
        assert!(cpu.flags.zero);
    }

    #[test]
    fn test_adczp() {
        let mut cpu = Cpu::new();
        let loc: usize = 0x64;
        cpu.memory[1] = loc as u8;
        cpu.memory[loc] = 23;
        cpu.flags.carry = true;
        cpu.exec_instruction(Ops::AdcZp);
        assert_eq!(cpu.rega, 24);
    }
}

