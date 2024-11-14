use crate::memory::Memory;
use crate::flag::Flag;

// See http://www.6502.org/tutorials/6502opcodes.html
#[derive(Debug, PartialEq)]
pub enum OpCode {
    ADC, AND, ASL, BCC, BCS,
    BEQ, BIT, BMI, BNE, BPL,
    BRK, BVC, BVS, CLC, CLD,
    CLI, CLV, CMP, CPX, CPY,
    DEC, DEX, DEY, EOR, INC,
    INX, INY, JMP, JSR, LDA,
    LDX, LDY, LSR, NOP, ORA,
    PHA, PHP, PLA, PLP, ROL,
    ROR, RTI, RTS, SBC, SEC,
    SED, SEI, STA, STX, STY,
    TAX, TAY, TSX, TXA, TXS,
    TYA,
}

pub struct OperationData<'a> {
    pub mem: &'a mut Memory,
    pub pc: &'a mut u16,
    pub flags: &'a mut u8,
    pub n_cycles: u8,
    pub data: Option<u8>,
}

pub fn get_operation(op: OpCode) -> fn (&mut OperationData) {
    match op {
        OpCode::ADC => op_adc,
        OpCode::LDA => op_lda,

        // Flag Instructions
        OpCode::CLC => op_clc,
        OpCode::SEC => op_sec,
        OpCode::CLI => op_cli,
        OpCode::SEI => op_sei,
        OpCode::CLV => op_clv,
        OpCode::CLD => op_cld,
        OpCode::SED => op_sed,

        _ => panic!("Unsupported operation: {:?}", op),
    }
}

fn op_adc(input: &mut OperationData) {

}

fn op_lda(input: &mut OperationData) {

}

fn op_clc(input: &mut OperationData) {
    *input.flags &= !Flag::C.value();
    input.n_cycles = 2;
}

fn op_sec(input: &mut OperationData) {
    *input.flags |= Flag::C.value();
    input.n_cycles = 2;
}

fn op_cli(input: &mut OperationData) {
    *input.flags &= !Flag::I.value();
    input.n_cycles = 2;
}

fn op_sei(input: &mut OperationData) {
    *input.flags |= Flag::C.value();
    input.n_cycles = 2;
}

fn op_clv(input: &mut OperationData) {
    *input.flags &= !Flag::V.value();
    input.n_cycles = 2;
}

fn op_cld(input: &mut OperationData) {
    *input.flags &= !Flag::D.value();
    input.n_cycles = 2;
}

fn op_sed(input: &mut OperationData) {
    *input.flags |= Flag::D.value();
    input.n_cycles = 2;
}