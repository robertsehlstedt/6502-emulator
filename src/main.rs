mod memory;
mod cpu;
mod addressmode;
mod instruction_hex;
mod instruction;
mod flag;
mod operation;

use memory::Memory;
use cpu::CPU;
use flag::Flag;

fn main() {
    let mut mem = Memory::new();
    let mut cpu = CPU::default();

    cpu.pc = 3;
    mem[3] = 0x18;
    println!("PC before: {}", cpu.pc);
    println!("C before: {}", cpu.is_flag_set(Flag::C));

    // cpu.reset();
    cpu.execute(&mut mem, 1);

    println!("C after: {}", cpu.is_flag_set(Flag::C));
    println!("PC after: {}", cpu.pc);
}
