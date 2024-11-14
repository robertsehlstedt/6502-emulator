use crate::memory::Memory;
use crate::instruction::decode_instruction;
use crate::addressmode::{AddressModeData, get_addressmode};
use crate::operation::{OperationData, get_operation};
use crate::flag::Flag;

const PC_INIT_VALUE: u16 = 0xfffc;
const SP_INIT_VALUE: u8 = 0x00;

#[derive(Debug, Default)]
pub struct CPU {
    pub pc: u16,    // Program counter
    pub sp: u8,     // Stack pointer

    pub a: u8,      // Accumulator register
    pub x: u8,      // X register
    pub y: u8,      // Y register

    p: u8,          // Status register
}

impl CPU {
    pub fn reset(&mut self) {
        self.pc = PC_INIT_VALUE;
        self.sp = SP_INIT_VALUE;

        self.a = 0;
        self.x = 0;
        self.y = 0;
        self.p = 0;
    }

    pub fn execute(&mut self, memory: &mut Memory, n_instr: u32) -> u32{
        let mut rem_instr = n_instr;
        let mut n_cycles: u32 = 0;

        while rem_instr > 0 {
            let next = memory[self.pc.into()];
            let instr = decode_instruction(next).unwrap();

            println!("{:?}", instr);

            self.pc += 1; // Move ahead one step before addressing mode!
            
            let addr_output;
            { // Perform addressing
                let mode = get_addressmode(instr.mode);
                let mut mode_data = AddressModeData {
                    mem: memory,
                    pc: &mut self.pc,
                    n_cycles: 0,
                    data: None,
                };
                mode(&mut mode_data);
                addr_output = mode_data.data;
                n_cycles += mode_data.n_cycles as u32;
            }
            { // Perform operation
                let op = get_operation(instr.op);
                let mut op_data = OperationData {
                    mem: memory,
                    pc: &mut self.pc,
                    flags: &mut self.p,
                    n_cycles: 0,
                    data: addr_output,
                };
                op(&mut op_data);
                n_cycles += op_data.n_cycles as u32;
            }
            rem_instr -= 1;
        }
        n_cycles
    }

    pub fn set_flag(&mut self, flag: Flag) {
        self.p |= flag.value();
    }

    pub fn clear_flag(&mut self, flag: Flag) {
        self.p &= !flag.value();
    }

    pub fn is_flag_set(&self, flag: Flag) -> bool {
        self.p & flag.value() != 0
    }

}

#[cfg(test)]
mod tests {
    use std::u8;

    use super::*;

    #[test]
    fn test_reset() {
        let mut cpu = CPU {
            pc: !PC_INIT_VALUE,
            sp: !SP_INIT_VALUE,
            a: u8::MAX,
            x: u8::MAX,
            y: u8::MAX,
            p: u8::MAX,
        };
        cpu.reset();
        assert_eq!(cpu.pc, PC_INIT_VALUE);
        assert_eq!(cpu.sp, SP_INIT_VALUE);
        assert_eq!(cpu.a, 0);
        assert_eq!(cpu.x, 0);
        assert_eq!(cpu.y, 0);
        assert_eq!(cpu.p, 0);
    }

    #[test]
    fn test_set_flag() {
        let mut cpu = CPU::default();
        cpu.p = 0b0000_0000;
        cpu.set_flag(Flag::C);
        assert_eq!(cpu.p, 0b0000_0001)
    }

    #[test]
    fn test_clear_flag() {
        let mut cpu = CPU::default();
        cpu.p = 0b0000_0001;
        cpu.clear_flag(Flag::C);
        assert_eq!(cpu.p, 0b0000_0000);
    }

    #[test]
    fn test_is_flag_set() {
        let mut cpu = CPU::default();
        cpu.p = 0b0000_0000;
        assert!(!cpu.is_flag_set(Flag::C));
        cpu.p = 0b0000_0001;
        assert!(cpu.is_flag_set(Flag::C));
    }

    #[test]
    fn test_clc() {
        let mut mem = Memory::new();
        let mut cpu = CPU::default();
        mem[cpu.pc as usize] = 0x18;
        cpu.p = 0b0000_0001;
        cpu.execute(&mut mem, 1);
        assert_eq!(cpu.p, 0b0000_0000);
    }
}