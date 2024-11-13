use crate::addressmode::AddrMode;
use crate::operation::OpCode;

#[derive(Debug)]
pub struct Instruction {
    pub op: OpCode,
    pub mode: AddrMode,
}

// See http://www.6502.org/tutorials/6502opcodes.html
pub fn get_instruction(instr: u8) -> Option<Instruction> {
    match instr {
        // ADC
        0x69 => Some(Instruction{op: OpCode::ADC, mode: AddrMode::IMM}),
        0x65 => Some(Instruction{op: OpCode::ADC, mode: AddrMode::ZPG}),
        0x75 => Some(Instruction{op: OpCode::ADC, mode: AddrMode::ZPX}),
        0x6D => Some(Instruction{op: OpCode::ADC, mode: AddrMode::ABS}),
        0x7D => Some(Instruction{op: OpCode::ADC, mode: AddrMode::ABX}),
        0x79 => Some(Instruction{op: OpCode::ADC, mode: AddrMode::ABY}),
        0x61 => Some(Instruction{op: OpCode::ADC, mode: AddrMode::INX}),
        0x71 => Some(Instruction{op: OpCode::ADC, mode: AddrMode::INY}),

        // AND
        0x29 => Some(Instruction{op: OpCode::AND, mode: AddrMode::IMM}),
        0x25 => Some(Instruction{op: OpCode::AND, mode: AddrMode::ZPG}),
        0x35 => Some(Instruction{op: OpCode::AND, mode: AddrMode::ZPX}),
        0x2D => Some(Instruction{op: OpCode::AND, mode: AddrMode::ABS}),
        0x3D => Some(Instruction{op: OpCode::AND, mode: AddrMode::ABX}),
        0x39 => Some(Instruction{op: OpCode::AND, mode: AddrMode::ABY}),
        0x21 => Some(Instruction{op: OpCode::AND, mode: AddrMode::INX}),
        0x31 => Some(Instruction{op: OpCode::AND, mode: AddrMode::INY}),

        // ASL
        0x0A => Some(Instruction{op: OpCode::ASL, mode: AddrMode::ACC}),
        0x06 => Some(Instruction{op: OpCode::ASL, mode: AddrMode::ZPG}),
        0x16 => Some(Instruction{op: OpCode::ASL, mode: AddrMode::ZPX}),
        0x0E => Some(Instruction{op: OpCode::ASL, mode: AddrMode::ABS}),
        0x1E => Some(Instruction{op: OpCode::ASL, mode: AddrMode::ABX}),

        // BIT
        0x24 => Some(Instruction{op: OpCode::BIT, mode: AddrMode::ZPG}),
        0x2C => Some(Instruction{op: OpCode::BIT, mode: AddrMode::ABS}),

        // Branch Instructions
        0x10 => Some(Instruction{op: OpCode::BPL, mode: AddrMode::REL}),
        0x30 => Some(Instruction{op: OpCode::BMI, mode: AddrMode::REL}),
        0x50 => Some(Instruction{op: OpCode::BVC, mode: AddrMode::REL}),
        0x70 => Some(Instruction{op: OpCode::BVS, mode: AddrMode::REL}),
        0x90 => Some(Instruction{op: OpCode::BCC, mode: AddrMode::REL}),
        0xB0 => Some(Instruction{op: OpCode::BCS, mode: AddrMode::REL}),
        0xD0 => Some(Instruction{op: OpCode::BNE, mode: AddrMode::REL}),
        0xF0 => Some(Instruction{op: OpCode::BEQ, mode: AddrMode::REL}),

        // BRK
        0x00 => Some(Instruction{op: OpCode::BRK, mode: AddrMode::IMP}),

        // CMP
        0xC9 => Some(Instruction{op: OpCode::CMP, mode: AddrMode::IMM}),
        0xC5 => Some(Instruction{op: OpCode::CMP, mode: AddrMode::ZPG}),
        0xD5 => Some(Instruction{op: OpCode::CMP, mode: AddrMode::ZPX}),
        0xCD => Some(Instruction{op: OpCode::CMP, mode: AddrMode::ABS}),
        0xDD => Some(Instruction{op: OpCode::CMP, mode: AddrMode::ABX}),
        0xD9 => Some(Instruction{op: OpCode::CMP, mode: AddrMode::ABY}),
        0xC1 => Some(Instruction{op: OpCode::CMP, mode: AddrMode::INX}),
        0xD1 => Some(Instruction{op: OpCode::CMP, mode: AddrMode::INY}),

        // CPX
        0xE0 => Some(Instruction{op: OpCode::CPX, mode: AddrMode::IMM}),
        0xE4 => Some(Instruction{op: OpCode::CPX, mode: AddrMode::ZPG}),
        0xEC => Some(Instruction{op: OpCode::CPX, mode: AddrMode::ABS}),
        
        // CPY
        0xC0 => Some(Instruction{op: OpCode::CPY, mode: AddrMode::IMM}),
        0xC4 => Some(Instruction{op: OpCode::CPY, mode: AddrMode::ZPG}),
        0xCC => Some(Instruction{op: OpCode::CPY, mode: AddrMode::ABS}),
        
        // DEC
        0xC6 => Some(Instruction{op: OpCode::DEC, mode: AddrMode::ZPG}),
        0xD6 => Some(Instruction{op: OpCode::DEC, mode: AddrMode::ZPX}),
        0xCE => Some(Instruction{op: OpCode::DEC, mode: AddrMode::ABS}),
        0xDE => Some(Instruction{op: OpCode::DEC, mode: AddrMode::ABX}),

        // EOR
        0x49 => Some(Instruction{op: OpCode::EOR, mode: AddrMode::IMM}),
        0x45 => Some(Instruction{op: OpCode::EOR, mode: AddrMode::ZPG}),
        0x55 => Some(Instruction{op: OpCode::EOR, mode: AddrMode::ZPX}),
        0x4D => Some(Instruction{op: OpCode::EOR, mode: AddrMode::ABS}),
        0x5D => Some(Instruction{op: OpCode::EOR, mode: AddrMode::ABX}),
        0x59 => Some(Instruction{op: OpCode::EOR, mode: AddrMode::ABY}),
        0x41 => Some(Instruction{op: OpCode::EOR, mode: AddrMode::INX}),
        0x51 => Some(Instruction{op: OpCode::EOR, mode: AddrMode::INY}),

        // Flag Instructions
        0x18 => Some(Instruction{op: OpCode::CLC, mode: AddrMode::IMP}),
        0x38 => Some(Instruction{op: OpCode::SEC, mode: AddrMode::IMP}),
        0x58 => Some(Instruction{op: OpCode::CLI, mode: AddrMode::IMP}),
        0x78 => Some(Instruction{op: OpCode::SEI, mode: AddrMode::IMP}),
        0xB8 => Some(Instruction{op: OpCode::CLV, mode: AddrMode::IMP}),
        0xD8 => Some(Instruction{op: OpCode::CLD, mode: AddrMode::IMP}),
        0xF8 => Some(Instruction{op: OpCode::SED, mode: AddrMode::IMP}),

        // INC
        0xE6 => Some(Instruction{op: OpCode::INC, mode: AddrMode::ZPG}),
        0xF6 => Some(Instruction{op: OpCode::INC, mode: AddrMode::ZPX}),
        0xEE => Some(Instruction{op: OpCode::INC, mode: AddrMode::ABS}),
        0xFE => Some(Instruction{op: OpCode::INC, mode: AddrMode::ABX}),

        // JMP
        0x4C => Some(Instruction{op: OpCode::JMP, mode: AddrMode::ABS}),
        0x6C => Some(Instruction{op: OpCode::JMP, mode: AddrMode::IND}),

        // JSR
        0x20 => Some(Instruction{op: OpCode::JSR, mode: AddrMode::ABS}),

        // LDA
        0xA9 => Some(Instruction{op: OpCode::LDA, mode: AddrMode::IMM}),
        0xA5 => Some(Instruction{op: OpCode::LDA, mode: AddrMode::ZPG}),
        0xB5 => Some(Instruction{op: OpCode::LDA, mode: AddrMode::ZPX}),
        0xAD => Some(Instruction{op: OpCode::LDA, mode: AddrMode::ABS}),
        0xBD => Some(Instruction{op: OpCode::LDA, mode: AddrMode::ABX}),
        0xB9 => Some(Instruction{op: OpCode::LDA, mode: AddrMode::ABY}),
        0xA1 => Some(Instruction{op: OpCode::LDA, mode: AddrMode::INX}),
        0xB1 => Some(Instruction{op: OpCode::LDA, mode: AddrMode::INY}),

        // LDX
        0xA2 => Some(Instruction{op: OpCode::LDX, mode: AddrMode::IMM}),
        0xA6 => Some(Instruction{op: OpCode::LDX, mode: AddrMode::ZPG}),
        0xB6 => Some(Instruction{op: OpCode::LDX, mode: AddrMode::ZPY}),
        0xAE => Some(Instruction{op: OpCode::LDX, mode: AddrMode::ABS}),
        0xBE => Some(Instruction{op: OpCode::LDX, mode: AddrMode::ABY}),

        // LDY
        0xA0 => Some(Instruction{op: OpCode::LDY, mode: AddrMode::IMM}),
        0xA4 => Some(Instruction{op: OpCode::LDY, mode: AddrMode::ZPG}),
        0xB4 => Some(Instruction{op: OpCode::LDY, mode: AddrMode::ZPX}),
        0xAC => Some(Instruction{op: OpCode::LDY, mode: AddrMode::ABS}),
        0xBC => Some(Instruction{op: OpCode::LDY, mode: AddrMode::ABX}),

        // LSR
        0x4A => Some(Instruction{op: OpCode::LSR, mode: AddrMode::ACC}),
        0x46 => Some(Instruction{op: OpCode::LSR, mode: AddrMode::ZPG}),
        0x56 => Some(Instruction{op: OpCode::LSR, mode: AddrMode::ZPX}),
        0x4E => Some(Instruction{op: OpCode::LSR, mode: AddrMode::ABS}),
        0x5E => Some(Instruction{op: OpCode::LSR, mode: AddrMode::ABX}),

        // NOP
        0xEA => Some(Instruction{op: OpCode::NOP, mode: AddrMode::IMP}),

        // ORA
        0x09 => Some(Instruction{op: OpCode::ORA, mode: AddrMode::IMM}),
        0x05 => Some(Instruction{op: OpCode::ORA, mode: AddrMode::ZPG}),
        0x15 => Some(Instruction{op: OpCode::ORA, mode: AddrMode::ZPX}),
        0x0D => Some(Instruction{op: OpCode::ORA, mode: AddrMode::ABS}),
        0x1D => Some(Instruction{op: OpCode::ORA, mode: AddrMode::ABX}),
        0x19 => Some(Instruction{op: OpCode::ORA, mode: AddrMode::ABY}),
        0x01 => Some(Instruction{op: OpCode::ORA, mode: AddrMode::INX}),
        0x11 => Some(Instruction{op: OpCode::ORA, mode: AddrMode::INY}),

        // Register Instructions
        0xAA => Some(Instruction{op: OpCode::TAX, mode: AddrMode::IMP}),
        0x8A => Some(Instruction{op: OpCode::TXA, mode: AddrMode::IMP}),
        0xCA => Some(Instruction{op: OpCode::DEX, mode: AddrMode::IMP}),
        0xE8 => Some(Instruction{op: OpCode::INX, mode: AddrMode::IMP}),
        0xA8 => Some(Instruction{op: OpCode::TAY, mode: AddrMode::IMP}),
        0x98 => Some(Instruction{op: OpCode::TYA, mode: AddrMode::IMP}),
        0x88 => Some(Instruction{op: OpCode::DEY, mode: AddrMode::IMP}),
        0xC8 => Some(Instruction{op: OpCode::INY, mode: AddrMode::IMP}),

        // ROL
        0x2A => Some(Instruction{op: OpCode::ROL, mode: AddrMode::ACC}),
        0x26 => Some(Instruction{op: OpCode::ROL, mode: AddrMode::ZPG}),
        0x36 => Some(Instruction{op: OpCode::ROL, mode: AddrMode::ZPX}),
        0x2E => Some(Instruction{op: OpCode::ROL, mode: AddrMode::ABS}),
        0x3E => Some(Instruction{op: OpCode::ROL, mode: AddrMode::ABX}),

        // ROR
        0x6A => Some(Instruction{op: OpCode::ROR, mode: AddrMode::ACC}),
        0x66 => Some(Instruction{op: OpCode::ROR, mode: AddrMode::ZPG}),
        0x76 => Some(Instruction{op: OpCode::ROR, mode: AddrMode::ZPX}),
        0x6E => Some(Instruction{op: OpCode::ROR, mode: AddrMode::ABS}),
        0x7E => Some(Instruction{op: OpCode::ROR, mode: AddrMode::ABX}),

        // RTI
        0x40 => Some(Instruction{op: OpCode::RTI, mode: AddrMode::IMP}),

        // RTS
        0x60 => Some(Instruction{op: OpCode::RTS, mode: AddrMode::IMP}),

        // SBC
        0xE9 => Some(Instruction{op: OpCode::SBC, mode: AddrMode::IMM}),
        0xE5 => Some(Instruction{op: OpCode::SBC, mode: AddrMode::ZPG}),
        0xF5 => Some(Instruction{op: OpCode::SBC, mode: AddrMode::ZPX}),
        0xED => Some(Instruction{op: OpCode::SBC, mode: AddrMode::ABS}),
        0xFD => Some(Instruction{op: OpCode::SBC, mode: AddrMode::ABX}),
        0xF9 => Some(Instruction{op: OpCode::SBC, mode: AddrMode::ABY}),
        0xE1 => Some(Instruction{op: OpCode::SBC, mode: AddrMode::INX}),
        0xF1 => Some(Instruction{op: OpCode::SBC, mode: AddrMode::INY}),

        // STA
        0x85 => Some(Instruction{op: OpCode::STA, mode: AddrMode::ZPG}),
        0x95 => Some(Instruction{op: OpCode::STA, mode: AddrMode::ZPX}),
        0x8D => Some(Instruction{op: OpCode::STA, mode: AddrMode::ABS}),
        0x9D => Some(Instruction{op: OpCode::STA, mode: AddrMode::ABX}),
        0x99 => Some(Instruction{op: OpCode::STA, mode: AddrMode::ABY}),
        0x81 => Some(Instruction{op: OpCode::STA, mode: AddrMode::INX}),
        0x91 => Some(Instruction{op: OpCode::STA, mode: AddrMode::INY}),

        // Stack Instructions
        0x9A => Some(Instruction{op: OpCode::TXS, mode: AddrMode::IMP}),
        0xBA => Some(Instruction{op: OpCode::TSX, mode: AddrMode::IMP}),
        0x48 => Some(Instruction{op: OpCode::PHA, mode: AddrMode::IMP}),
        0x68 => Some(Instruction{op: OpCode::PLA, mode: AddrMode::IMP}),
        0x08 => Some(Instruction{op: OpCode::PHP, mode: AddrMode::IMP}),
        0x28 => Some(Instruction{op: OpCode::PLP, mode: AddrMode::IMP}),

        // STX
        0x86 => Some(Instruction{op: OpCode::STX, mode: AddrMode::ZPG}),
        0x96 => Some(Instruction{op: OpCode::STX, mode: AddrMode::ZPY}),
        0x8E => Some(Instruction{op: OpCode::STX, mode: AddrMode::ABS}),

        // STY
        0x84 => Some(Instruction{op: OpCode::STY, mode: AddrMode::ZPG}),
        0x94 => Some(Instruction{op: OpCode::STY, mode: AddrMode::ZPX}),
        0x8C => Some(Instruction{op: OpCode::STY, mode: AddrMode::ABS}),

        _ => None,
    }
}
