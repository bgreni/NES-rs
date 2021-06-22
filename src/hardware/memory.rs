use crate::hardware::cpu::Cpu;
use crate::utils::{combine_bytes, split_bytes};

pub const MEM_SIZE: usize = 0xFFFF + 1;
// pub const ZERO_PAGE_BOUND: usize = 0xFF;
// pub const INTERNAL_BOUND: u16 = 0x07FF;

pub trait MemoryOps {
    fn fetch_next_byte(&mut self) -> u8;
    fn fetch_two_bytes(&mut self) -> (u8, u8);
    fn fetch_zp(&mut self) -> usize;
    fn fetch_zpx(&mut self) -> usize;
    fn fetch_abs(&mut self) -> usize;
    fn fetch_absx(&mut self) -> usize;
    fn fetch_absy(&mut self) -> usize;
    fn fetch_indirectx(&mut self) -> usize;
    fn fetch_indirecty(&mut self) -> usize;
    fn save_pc(&mut self);
    fn save_status(&mut self);
}

impl MemoryOps for Cpu {
    fn fetch_next_byte(&mut self) -> u8 {
        self.pc += 1;
        return self.memory[self.pc as usize];
    }

    fn fetch_two_bytes(&mut self) -> (u8, u8) {
        let (p1, p2) = (self.pc + 1, self.pc + 2);
        return (self.memory[p1 as usize], self.memory[p2 as usize])

    }

    fn fetch_zp(&mut self) -> usize {
        return self.fetch_next_byte().into();
    }

    fn fetch_zpx(&mut self) -> usize {
        return (self.fetch_next_byte().wrapping_add(self.regx)).into();
    }

    fn fetch_abs(&mut self) -> usize {
        let (upper, lower) = self.fetch_two_bytes();
        return combine_bytes(upper.into(), lower.into()).into();
    }

    fn fetch_absx(&mut self) -> usize {
        let (upper, lower) = self.fetch_two_bytes();
        let addr: usize = combine_bytes(upper.into(), lower.into()).into();
        return addr + self.regx as usize;
    }

    fn fetch_absy(&mut self) -> usize {
        let (upper, lower) = self.fetch_two_bytes();
        let addr: usize = combine_bytes(upper.into(), lower.into()).into();
        return addr + self.regy as usize;
    }

    fn fetch_indirectx(&mut self) -> usize {
        let zp_addr: usize = (self.fetch_next_byte().wrapping_add(self.regx)).into();
        let base: usize = self.memory[zp_addr].into();
        let (upper, lower) = (self.memory[base+1], self.memory[base]);
        return combine_bytes(upper.into(), lower.into()).into();
    }

    fn fetch_indirecty(&mut self) -> usize {
        let zp_addr: usize = (self.fetch_next_byte().wrapping_add(self.regy)).into();
        let base: usize = self.memory[zp_addr].into();
        let (upper, lower) = (self.memory[base+1], self.memory[base]);
        return combine_bytes(upper.into(), lower.into()).into();
    }

    fn save_pc(&mut self) {
        let (upper, lower) = split_bytes(self.pc);
        self.stack.push(lower);
        self.stack.push(upper);
    }

    fn save_status(&mut self) {
        self.stack.push(self.flags.to_u8());
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_save_pc() {
        let mut cpu = Cpu::new();
        cpu.pc = 0x3456;
        cpu.save_pc();
        assert_eq!(cpu.stack[0], 0x56);
        assert_eq!(cpu.stack[1], 0x34);
    }

    #[test]
    fn test_fetch_indirecty() {
        let mut cpu = Cpu::new();
        cpu.memory[0x14] = 0x24;
        cpu.memory[0x24] = 0x45;
        cpu.memory[0x25] = 0x34;
        cpu.memory[1] = 0x14 - 10;
        cpu.regy = 10;
        assert_eq!(cpu.fetch_indirecty(), 0x3445);
    }

    #[test]
    fn test_fetch_indirecty_wrap() {
        let mut cpu = Cpu::new();
        cpu.memory[0x14] = 0x24;
        cpu.memory[0x24] = 0x45;
        cpu.memory[0x25] = 0x34;
        cpu.memory[1] = 0x15;
        cpu.regy = 0xFF;
        assert_eq!(cpu.fetch_indirecty(), 0x3445);
    }

    #[test]
    fn test_fetch_indirectx_wrap() {
        let mut cpu = Cpu::new();
        cpu.memory[0x14] = 0x24;
        cpu.memory[0x24] = 0x45;
        cpu.memory[0x25] = 0x34;
        cpu.memory[1] = 0x15;
        cpu.regx = 0xFF;
        assert_eq!(cpu.fetch_indirectx(), 0x3445);
    }

    #[test]
    fn test_fetch_indirectx() {
        let mut cpu = Cpu::new();
        cpu.memory[0x14] = 0x24;
        cpu.memory[0x24] = 0x45;
        cpu.memory[0x25] = 0x34;
        cpu.memory[1] = 0x14 - 10;
        cpu.regx = 10;
        assert_eq!(cpu.fetch_indirectx(), 0x3445);
    }

    #[test]
    fn test_fetch_absy() {
        let mut cpu = Cpu::new();
        cpu.memory[1] = 0x11;
        cpu.memory[2] = 0x11;
        cpu.regy = 10;
        assert_eq!(cpu.fetch_absy(), 0x1111 + 10);
    }

    #[test]
    fn test_fetch_absx() {
        let mut cpu = Cpu::new();
        cpu.memory[1] = 0x11;
        cpu.memory[2] = 0x11;
        cpu.regx = 10;
        assert_eq!(cpu.fetch_absx(), 0x1111 + 10);
    }

    #[test]
    fn test_fetch_two_bytes() {
        let mut cpu = Cpu::new();
        cpu.memory[1] = 10;
        cpu.memory[2] = 20;
        assert_eq!(cpu.fetch_two_bytes(), (10, 20));
    }

    #[test]
    fn test_fetch_abs() {
        let mut cpu = Cpu::new();
        cpu.memory[1] = 0x11;
        cpu.memory[2] = 0x11;
        assert_eq!(cpu.fetch_abs(), 0x1111);
    }

    #[test]
    fn test_fetch_byte() {
        let mut cpu = Cpu::new();
        cpu.memory[1] = 23;
        assert_eq!(cpu.fetch_next_byte(), 23u8);
    }

    #[test]
    fn test_fetch_zp() {
        let mut cpu = Cpu::new();
        cpu.memory[1] = 10;
        assert_eq!(cpu.fetch_zp(), 10);
    }

    #[test]
    fn test_fetch_zpx() {
        let mut cpu = Cpu::new();
        cpu.memory[1] = 8;
        cpu.regx = 2;
        assert_eq!(cpu.fetch_zpx(), 10);
    }

    #[test]
    fn test_fetch_zpx_wrap() {
        let mut cpu = Cpu::new();
        cpu.memory[1] = 0x80;
        cpu.regx = 0xFF;
        assert_eq!(cpu.fetch_zpx(), 0x7F);
    }
}