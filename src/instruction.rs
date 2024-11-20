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

pub type Instruction = (InstructionCode, OperationInput);

pub fn get_op_input(am: AddressingMode) -> OperationInput {
    match am {
        AddressingMode::ACC | AddressingMode::IMP => {
            OperationInput::IMP
        }
        AddressingMode::IMM => {
            OperationInput::IMM(0)
        }
        AddressingMode::ZPG => {
            OperationInput::ADR(0)
        }
        AddressingMode::ZPX => {
            OperationInput::ADR(0)
        }
        AddressingMode::ZPY => {
            OperationInput::ADR(0)
        }
        AddressingMode::REL => {
            OperationInput::REL(0)
        }
        AddressingMode::ABS => {
            OperationInput::ADR(0)
        }
        AddressingMode::ABX => {
            OperationInput::ADR(0)
        }
        AddressingMode::ABY => {
            OperationInput::ADR(0)
        }
        AddressingMode::IND => {
            OperationInput::ADR(0)
        }
        AddressingMode::INX => {
            OperationInput::ADR(0)
        }
        AddressingMode::INY => {
            OperationInput::ADR(0)
        }
    }
}