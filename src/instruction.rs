#[derive(Debug, Copy, Clone)]
pub enum AddressingMode {
    IMP, ACC, IMM, ZPG, ZPX,
    ZPY, REL, ABS, ABX, ABY,
    IND, INX, INY,
}

#[derive(Debug, Copy, Clone)]
pub enum InstructionCode {
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

#[derive(Debug, Copy, Clone)]
pub enum OperationInput {
    IMP,
    IMM(u8),
    REL(u16),
    ADR(u16),
}

pub type Instruction = (InstructionCode, AddressingMode, OperationInput);