use cpu_6502::{nmos6502::Nmos6502, Bus, Cpu};

struct Memory([u8; 65536]);
impl Bus for Memory {
    fn read(&self, addr: u16) -> u8 { self.0[addr as usize] }
    fn write(&mut self, addr: u16, value: u8) { self.0[addr as usize] = value }
}

fn main() {
    let mut mem = Memory([0; 65536]);
    mem.write(0xFFFC, 0xA0); // Set PC to A0 on reset

    let mut cpu = Cpu::new(Nmos6502);
    cpu.pc = 0x00A0;
    cpu.sp = 0x01;

    mem.write(0x00A0, 0x18); // CLC
    cpu.reg.c = true;
    cpu.step(&mut mem);

    println!("Carry: {:?}", cpu.reg.c);
    println!("PC: {:04x}", cpu.pc);

    mem.write(0x00A1, 0x6C); // JMP IND
    mem.write(0x00A2, 0xBB);
    mem.write(0x00A3, 0xCC);

    mem.write(0xCCBB, 0xCD);
    mem.write(0xCCBC, 0xAB);

    cpu.step(&mut mem);

    println!("PC: {:04x}", cpu.pc);

    // c.step(&mut m);
}
