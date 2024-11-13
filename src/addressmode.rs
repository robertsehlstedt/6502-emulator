use crate::memory::Memory;

// See https://www.nesdev.org/obelisk-6502-guide/addressing.html
#[derive(Debug)]
pub enum AddrMode {
    IMP, ACC, IMM, ZPG, ZPX,
    ZPY, REL, ABS, ABX, ABY,
    IND, INX, INY,
}

pub struct AddressModeData<'a> {
    pub mem: &'a mut Memory,
    pub pc: &'a mut u16,
    pub n_cycles: u8,
    pub data: Option<u8>,
}

pub fn get_addressmode(mode: AddrMode) -> fn (&mut AddressModeData) {
    match mode {
        AddrMode::IMP => mode_imp,
        AddrMode::IMM => mode_imm,
        _ => panic!("Unsupported mode: {:?}", mode),
    }
}

fn mode_imp(data: &mut AddressModeData) {
    println!("mode_imp");
    *data.pc += 1;
}

fn mode_imm(data: &mut AddressModeData) {
    println!("mode_imm");
    data.n_cycles = 1;
    data.data = Some(data.mem[*data.pc as usize]);
    *data.pc += 1;
}