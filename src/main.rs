use cpu_6502::{cpu::{Bus, Cpu}, instruction::{AddressingMode, InstructionCode}, Variant};

struct MyCPU;
impl Variant for MyCPU {
    fn decode(opcode: u8) -> Option<(
            cpu_6502::instruction::InstructionCode,
            cpu_6502::instruction::AddressingMode
        )> {
        Some((InstructionCode::ADC, AddressingMode::IMM))
    }
}

struct Memory;
impl Bus for Memory {
    fn read(&mut self, addr: u16) -> u8 {
        todo!()
    }

    fn write(&mut self, addr: u16, value: u8) {
        todo!()
    }
}

fn main() {
    // let mut c = Cpu::new(MyCPU);
    // let mut m= Memory{};
    // c.step(&mut m);
    let n: u8 = 0xFF;
}
