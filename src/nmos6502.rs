#[derive(Debug, Copy, Clone)]
pub struct Nmos6502;

impl crate::Variant for Nmos6502 {
    fn decode(opcode: u8) -> Option<(InstructionCode, AddressingMode)> {
        match opcode {
            0x00 => Some((InstructionCode::BRK, AddressingMode::IMP)),
            0x01 => Some((InstructionCode::ORA, AddressingMode::INX)),
            0x02 => None,
            0x03 => None,
            0x04 => None,
            0x05 => Some((InstructionCode::ORA, AddressingMode::ZPG)),
            0x06 => Some((InstructionCode::ASL, AddressingMode::ZPG)),
            0x07 => None,
            0x08 => Some((InstructionCode::PHP, AddressingMode::IMP)),
            0x09 => Some((InstructionCode::ORA, AddressingMode::IMM)),
            0x0a => Some((InstructionCode::ASL, AddressingMode::ACC)),
            0x0b => None,
            0x0c => None,
            0x0d => Some((InstructionCode::ORA, AddressingMode::ABS)),
            0x0e => Some((InstructionCode::ASL, AddressingMode::ABS)),
            0x0f => None,
            0x10 => Some((InstructionCode::BPL, AddressingMode::REL)),
            0x11 => Some((InstructionCode::ORA, AddressingMode::INY)),
            0x12 => None,
            0x13 => None,
            0x14 => None,
            0x15 => Some((InstructionCode::ORA, AddressingMode::ZPX)),
            0x16 => Some((InstructionCode::ASL, AddressingMode::ZPX)),
            0x17 => None,
            0x18 => Some((InstructionCode::CLC, AddressingMode::IMP)),
            0x19 => Some((InstructionCode::ORA, AddressingMode::ABY)),
            0x1a => None,
            0x1b => None,
            0x1c => None,
            0x1d => Some((InstructionCode::ORA, AddressingMode::ABX)),
            0x1e => Some((InstructionCode::ASL, AddressingMode::ABX)),
            0x1f => None,
            0x20 => Some((InstructionCode::JSR, AddressingMode::ABS)),
            0x21 => Some((InstructionCode::AND, AddressingMode::INX)),
            0x22 => None,
            0x23 => None,
            0x24 => Some((InstructionCode::BIT, AddressingMode::ZPG)),
            0x25 => Some((InstructionCode::AND, AddressingMode::ZPG)),
            0x26 => Some((InstructionCode::ROL, AddressingMode::ZPG)),
            0x27 => None,
            0x28 => Some((InstructionCode::PLP, AddressingMode::IMP)),
            0x29 => Some((InstructionCode::AND, AddressingMode::IMM)),
            0x2a => Some((InstructionCode::ROL, AddressingMode::ACC)),
            0x2b => None,
            0x2c => Some((InstructionCode::BIT, AddressingMode::ABS)),
            0x2d => Some((InstructionCode::AND, AddressingMode::ABS)),
            0x2e => Some((InstructionCode::ROL, AddressingMode::ABS)),
            0x2f => None,
            0x30 => Some((InstructionCode::BMI, AddressingMode::REL)),
            0x31 => Some((InstructionCode::AND, AddressingMode::INY)),
            0x32 => None,
            0x33 => None,
            0x34 => None,
            0x35 => Some((InstructionCode::AND, AddressingMode::ZPX)),
            0x36 => Some((InstructionCode::ROL, AddressingMode::ZPX)),
            0x37 => None,
            0x38 => Some((InstructionCode::SEC, AddressingMode::IMP)),
            0x39 => Some((InstructionCode::AND, AddressingMode::ABY)),
            0x3a => None,
            0x3b => None,
            0x3c => None,
            0x3d => Some((InstructionCode::AND, AddressingMode::ABX)),
            0x3e => Some((InstructionCode::ROL, AddressingMode::ABX)),
            0x3f => None,
            0x40 => Some((InstructionCode::RTI, AddressingMode::IMP)),
            0x41 => Some((InstructionCode::EOR, AddressingMode::INX)),
            0x42 => None,
            0x43 => None,
            0x44 => None,
            0x45 => Some((InstructionCode::EOR, AddressingMode::ZPG)),
            0x46 => Some((InstructionCode::LSR, AddressingMode::ZPG)),
            0x47 => None,
            0x48 => Some((InstructionCode::PHA, AddressingMode::IMP)),
            0x49 => Some((InstructionCode::EOR, AddressingMode::IMM)),
            0x4a => Some((InstructionCode::LSR, AddressingMode::ACC)),
            0x4b => None,
            0x4c => Some((InstructionCode::JMP, AddressingMode::ABS)),
            0x4d => Some((InstructionCode::EOR, AddressingMode::ABS)),
            0x4e => Some((InstructionCode::LSR, AddressingMode::ABS)),
            0x4f => None,
            0x50 => Some((InstructionCode::BVC, AddressingMode::REL)),
            0x51 => Some((InstructionCode::EOR, AddressingMode::INY)),
            0x52 => None,
            0x53 => None,
            0x54 => None,
            0x55 => Some((InstructionCode::EOR, AddressingMode::ZPX)),
            0x56 => Some((InstructionCode::LSR, AddressingMode::ZPX)),
            0x57 => None,
            0x58 => Some((InstructionCode::CLI, AddressingMode::IMP)),
            0x59 => Some((InstructionCode::EOR, AddressingMode::ABY)),
            0x5a => None,
            0x5b => None,
            0x5c => None,
            0x5d => Some((InstructionCode::EOR, AddressingMode::ABX)),
            0x5e => Some((InstructionCode::LSR, AddressingMode::ABX)),
            0x5f => None,
            0x60 => Some((InstructionCode::RTS, AddressingMode::IMP)),
            0x61 => Some((InstructionCode::ADC, AddressingMode::INX)),
            0x62 => None,
            0x63 => None,
            0x64 => None,
            0x65 => Some((InstructionCode::ADC, AddressingMode::ZPG)),
            0x66 => Some((InstructionCode::ROR, AddressingMode::ZPG)),
            0x67 => None,
            0x68 => Some((InstructionCode::PLA, AddressingMode::IMP)),
            0x69 => Some((InstructionCode::ADC, AddressingMode::IMM)),
            0x6a => Some((InstructionCode::ROR, AddressingMode::ACC)),
            0x6b => None,
            0x6c => Some((InstructionCode::JMP, AddressingMode::IND)),
            0x6d => Some((InstructionCode::ADC, AddressingMode::ABS)),
            0x6e => Some((InstructionCode::ROR, AddressingMode::ABS)),
            0x6f => None,
            0x70 => Some((InstructionCode::BVS, AddressingMode::REL)),
            0x71 => Some((InstructionCode::ADC, AddressingMode::INY)),
            0x72 => None,
            0x73 => None,
            0x74 => None,
            0x75 => Some((InstructionCode::ADC, AddressingMode::ZPX)),
            0x76 => Some((InstructionCode::ROR, AddressingMode::ZPX)),
            0x77 => None,
            0x78 => Some((InstructionCode::SEI, AddressingMode::IMP)),
            0x79 => Some((InstructionCode::ADC, AddressingMode::ABY)),
            0x7a => None,
            0x7b => None,
            0x7c => None,
            0x7d => Some((InstructionCode::ADC, AddressingMode::ABX)),
            0x7e => Some((InstructionCode::ROR, AddressingMode::ABX)),
            0x7f => None,
            0x80 => None,
            0x81 => Some((InstructionCode::STA, AddressingMode::INX)),
            0x82 => None,
            0x83 => None,
            0x84 => Some((InstructionCode::STY, AddressingMode::ZPG)),
            0x85 => Some((InstructionCode::STA, AddressingMode::ZPG)),
            0x86 => Some((InstructionCode::STX, AddressingMode::ZPG)),
            0x87 => None,
            0x88 => Some((InstructionCode::DEY, AddressingMode::IMP)),
            0x89 => None,
            0x8a => Some((InstructionCode::TXA, AddressingMode::IMP)),
            0x8b => None,
            0x8c => Some((InstructionCode::STY, AddressingMode::ABS)),
            0x8d => Some((InstructionCode::STA, AddressingMode::ABS)),
            0x8e => Some((InstructionCode::STX, AddressingMode::ABS)),
            0x8f => None,
            0x90 => Some((InstructionCode::BCC, AddressingMode::REL)),
            0x91 => Some((InstructionCode::STA, AddressingMode::INY)),
            0x92 => None,
            0x93 => None,
            0x94 => Some((InstructionCode::STY, AddressingMode::ZPX)),
            0x95 => Some((InstructionCode::STA, AddressingMode::ZPX)),
            0x96 => Some((InstructionCode::STX, AddressingMode::ZPY)),
            0x97 => None,
            0x98 => Some((InstructionCode::TYA, AddressingMode::IMP)),
            0x99 => Some((InstructionCode::STA, AddressingMode::ABY)),
            0x9a => Some((InstructionCode::TXS, AddressingMode::IMP)),
            0x9b => None,
            0x9c => None,
            0x9d => Some((InstructionCode::STA, AddressingMode::ABX)),
            0x9e => None,
            0x9f => None,
            0xa0 => Some((InstructionCode::LDY, AddressingMode::IMM)),
            0xa1 => Some((InstructionCode::LDA, AddressingMode::INX)),
            0xa2 => Some((InstructionCode::LDX, AddressingMode::IMM)),
            0xa3 => None,
            0xa4 => Some((InstructionCode::LDY, AddressingMode::ZPG)),
            0xa5 => Some((InstructionCode::LDA, AddressingMode::ZPG)),
            0xa6 => Some((InstructionCode::LDX, AddressingMode::ZPG)),
            0xa7 => None,
            0xa8 => Some((InstructionCode::TAY, AddressingMode::IMP)),
            0xa9 => Some((InstructionCode::LDA, AddressingMode::IMM)),
            0xaa => Some((InstructionCode::TAX, AddressingMode::IMP)),
            0xab => None,
            0xac => Some((InstructionCode::LDY, AddressingMode::ABS)),
            0xad => Some((InstructionCode::LDA, AddressingMode::ABS)),
            0xae => Some((InstructionCode::LDX, AddressingMode::ABS)),
            0xaf => None,
            0xb0 => Some((InstructionCode::BCS, AddressingMode::REL)),
            0xb1 => Some((InstructionCode::LDA, AddressingMode::INY)),
            0xb2 => None,
            0xb3 => None,
            0xb4 => Some((InstructionCode::LDY, AddressingMode::ZPX)),
            0xb5 => Some((InstructionCode::LDA, AddressingMode::ZPX)),
            0xb6 => Some((InstructionCode::LDX, AddressingMode::ZPY)),
            0xb7 => None,
            0xb8 => Some((InstructionCode::CLV, AddressingMode::IMP)),
            0xb9 => Some((InstructionCode::LDA, AddressingMode::ABY)),
            0xba => Some((InstructionCode::TSX, AddressingMode::IMP)),
            0xbb => None,
            0xbc => Some((InstructionCode::LDY, AddressingMode::ABX)),
            0xbd => Some((InstructionCode::LDA, AddressingMode::ABX)),
            0xbe => Some((InstructionCode::LDX, AddressingMode::ABY)),
            0xbf => None,
            0xc0 => Some((InstructionCode::CPY, AddressingMode::IMM)),
            0xc1 => Some((InstructionCode::CMP, AddressingMode::INX)),
            0xc2 => None,
            0xc3 => None,
            0xc4 => Some((InstructionCode::CPY, AddressingMode::ZPG)),
            0xc5 => Some((InstructionCode::CMP, AddressingMode::ZPG)),
            0xc6 => Some((InstructionCode::DEC, AddressingMode::ZPG)),
            0xc7 => None,
            0xc8 => Some((InstructionCode::INY, AddressingMode::IMP)),
            0xc9 => Some((InstructionCode::CMP, AddressingMode::IMM)),
            0xca => Some((InstructionCode::DEX, AddressingMode::IMP)),
            0xcb => None,
            0xcc => Some((InstructionCode::CPY, AddressingMode::ABS)),
            0xcd => Some((InstructionCode::CMP, AddressingMode::ABS)),
            0xce => Some((InstructionCode::DEC, AddressingMode::ABS)),
            0xcf => None,
            0xd0 => Some((InstructionCode::BNE, AddressingMode::REL)),
            0xd1 => Some((InstructionCode::CMP, AddressingMode::INY)),
            0xd2 => None,
            0xd3 => None,
            0xd4 => None,
            0xd5 => Some((InstructionCode::CMP, AddressingMode::ZPX)),
            0xd6 => Some((InstructionCode::DEC, AddressingMode::ZPX)),
            0xd7 => None,
            0xd8 => Some((InstructionCode::CLD, AddressingMode::IMP)),
            0xd9 => Some((InstructionCode::CMP, AddressingMode::ABY)),
            0xda => None,
            0xdb => None,
            0xdc => None,
            0xdd => Some((InstructionCode::CMP, AddressingMode::ABX)),
            0xde => Some((InstructionCode::DEC, AddressingMode::ABX)),
            0xdf => None,
            0xe0 => Some((InstructionCode::CPX, AddressingMode::IMM)),
            0xe1 => Some((InstructionCode::SBC, AddressingMode::INX)),
            0xe2 => None,
            0xe3 => None,
            0xe4 => Some((InstructionCode::CPX, AddressingMode::ZPG)),
            0xe5 => Some((InstructionCode::SBC, AddressingMode::ZPG)),
            0xe6 => Some((InstructionCode::INC, AddressingMode::ZPG)),
            0xe7 => None,
            0xe8 => Some((InstructionCode::INX, AddressingMode::IMP)),
            0xe9 => Some((InstructionCode::SBC, AddressingMode::IMM)),
            0xea => Some((InstructionCode::NOP, AddressingMode::IMP)),
            0xeb => None,
            0xec => Some((InstructionCode::CPX, AddressingMode::ABS)),
            0xed => Some((InstructionCode::SBC, AddressingMode::ABS)),
            0xee => Some((InstructionCode::INC, AddressingMode::ABS)),
            0xef => None,
            0xf0 => Some((InstructionCode::BEQ, AddressingMode::REL)),
            0xf1 => Some((InstructionCode::SBC, AddressingMode::INY)),
            0xf2 => None,
            0xf3 => None,
            0xf4 => None,
            0xf5 => Some((InstructionCode::SBC, AddressingMode::ZPX)),
            0xf6 => Some((InstructionCode::INC, AddressingMode::ZPX)),
            0xf7 => None,
            0xf8 => Some((InstructionCode::SED, AddressingMode::IMP)),
            0xf9 => Some((InstructionCode::SBC, AddressingMode::ABY)),
            0xfa => None,
            0xfb => None,
            0xfc => None,
            0xfd => Some((InstructionCode::SBC, AddressingMode::ABX)),
            0xfe => Some((InstructionCode::INC, AddressingMode::ABX)),
            0xff => None,
        }
    }
}