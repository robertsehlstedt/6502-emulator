use crate::memory::Memory;
use crate::instruction::get_instruction;
use crate::addressmode::{AddressModeData, get_addressmode};
use crate::operation::{OperationData, get_operation};
use crate::flag::Flag;

#[derive(Debug, Default)]
pub struct CPU {
    pub pc: u16,
    pub sp: u8,

    a: u8,
    x: u8,
    y: u8,
    p: u8,
}

impl CPU {
    pub fn reset(&mut self) {
        self.pc = 0xfffc;
        self.sp = 0x00;

        self.a = 0;
        self.x = 0;
        self.y = 0;
        self.p = 1;
    }

    pub fn execute(&mut self, memory: &mut Memory, n_instr: u32) -> u32{
        let mut rem_instr = n_instr;
        let mut n_cycles: u32 = 0;

        while rem_instr > 0 {
            let next = memory[self.pc.into()];
            let instr = get_instruction(next).unwrap();

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

    pub fn is_flag_set(&self, flag: Flag) -> bool {
        self.p & flag.value() != 0
    }

}