use crate::hardware::flags::Flags;
use crate::hardware::instruction::{AddrModes, Ops};
use crate::hardware::memory::{MEM_SIZE, MemoryOps};
use crate::utils::{get_top_bit, is_overflow, check_bit, combine_bytes};
use num_traits::FromPrimitive;


pub struct Cpu {
    pub (super) memory: Vec<u8>,
    rega: u8,
    pub (super) regx: u8,
    pub (super) regy: u8,
    pub (super) flags: Flags,
    pub (super) stack: Vec<u8>,
    pub (super) pc: u16,
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
            stack: Vec::new(),
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

            Ops::CmpI => self.cmp(AddrModes::Immediate),
            Ops::CmpZp => self.cmp(AddrModes::ZeroPage),
            Ops::CmpZpX => self.cmp(AddrModes::ZeroPageX),
            Ops::CmpAbs => self.cmp(AddrModes::Absolute),
            Ops::CmpAbsX => self.cmp(AddrModes::AbsoluteX),
            Ops::CmpAbsY => self.cmp(AddrModes::AbsoluteY),
            Ops::CmpIndX => self.cmp(AddrModes::IndirectX),
            Ops::CmpIndY => self.cmp(AddrModes::IndirectY),

            _ => self.cry(op as u8)
        }
    }

    fn cry(&self, op: u8) {
        panic!("Invalid opcode given: {}", op);
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

    fn cmp(&mut self, mode: AddrModes) {
        let val = self.get_value(mode);
        self.flags.carry = self.rega >= val;
        self.flags.zero = self.rega == val;
        self.flags.negative = self.rega < val;
    }

    fn bvs_or_bvc(&mut self, check_for_set: bool) {
        let displace: u16 = self.fetch_next_byte().into();

        if self.flags.overflow == check_for_set {
            self.pc += displace;
        }
    }

    fn brk(&mut self) {
        self.save_pc();
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
                println!("CARRY: {}", carry);
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
        let (res, carry) = self.rega.overflowing_add(val);
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
        assert_eq!(cpu.stack[0], 0x56);
        assert_eq!(cpu.stack[1], 0x34);
        assert_eq!(cpu.stack[2], 1);
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
        cpu.exec_instruction(Ops::AdcZp);
        assert_eq!(cpu.rega, 23);
    }
}

