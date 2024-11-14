// See http://www.6502.org/tutorials/6502opcodes.html

// ADC
pub const ADC_IMM: u8 = 0x69;
pub const ADC_ZPG: u8 = 0x65;
pub const ADC_ZPX: u8 = 0x75;
pub const ADC_ABS: u8 = 0x6D;
pub const ADC_ABX: u8 = 0x7D;
pub const ADC_ABY: u8 = 0x79;
pub const ADC_INX: u8 = 0x61;
pub const ADC_INY: u8 = 0x71;

// AND
pub const AND_IMM: u8 = 0x29;
pub const AND_ZPG: u8 = 0x25;
pub const AND_ZPX: u8 = 0x35;
pub const AND_ABS: u8 = 0x2D;
pub const AND_ABX: u8 = 0x3D;
pub const AND_ABY: u8 = 0x39;
pub const AND_INX: u8 = 0x21;
pub const AND_INY: u8 = 0x31;

// ASL
pub const ASL_ACC: u8 = 0x0A;
pub const ASL_ZPG: u8 = 0x06;
pub const ASL_ZPX: u8 = 0x16;
pub const ASL_ABS: u8 = 0x0E;
pub const ASL_ABX: u8 = 0x1E;

// BIT
pub const BIT_ZPG: u8 = 0x24;
pub const BIT_ABS: u8 = 0x2C;

// Branch Instructions
pub const BPL_REL: u8 = 0x10;
pub const BMI_REL: u8 = 0x30;
pub const BVC_REL: u8 = 0x50;
pub const BVS_REL: u8 = 0x70;
pub const BCC_REL: u8 = 0x90;
pub const BCS_REL: u8 = 0xB0;
pub const BNE_REL: u8 = 0xD0;
pub const BEQ_REL: u8 = 0xF0;

// BRK
pub const BRK_IMP: u8 = 0x00;

// CMP
pub const CMP_IMM: u8 = 0xC9;
pub const CMP_ZPG: u8 = 0xC5;
pub const CMP_ZPX: u8 = 0xD5;
pub const CMP_ABS: u8 = 0xCD;
pub const CMP_ABX: u8 = 0xDD;
pub const CMP_ABY: u8 = 0xD9;
pub const CMP_INX: u8 = 0xC1;
pub const CMP_INY: u8 = 0xD1;

// CPX
pub const CPX_IMM: u8 = 0xE0;
pub const CPX_ZPG: u8 = 0xE4;
pub const CPX_ABS: u8 = 0xEC;

// CPX
pub const CPY_IMM: u8 = 0xC0;
pub const CPY_ZPG: u8 = 0xC4;
pub const CPY_ABS: u8 = 0xCC;
        
// DEC
pub const DEC_ZPG: u8 = 0xC6;
pub const DEC_ZPX: u8 = 0xD6;
pub const DEC_ABS: u8 = 0xCE;
pub const DEC_ABX: u8 = 0xDE;

// EOR
pub const EOR_IMM: u8 = 0x49;
pub const EOR_ZPG: u8 = 0x45;
pub const EOR_ZPX: u8 = 0x55;
pub const EOR_ABS: u8 = 0x4D;
pub const EOR_ABX: u8 = 0x5D;
pub const EOR_ABY: u8 = 0x59;
pub const EOR_INX: u8 = 0x41;
pub const EOR_INY: u8 = 0x51;

// Flag Instructions
pub const CLC_IMP: u8 = 0x18;
pub const SEC_IMP: u8 = 0x38;
pub const CLI_IMP: u8 = 0x58;
pub const SEI_IMP: u8 = 0x78;
pub const CLV_IMP: u8 = 0xB8;
pub const CLD_IMP: u8 = 0xD8;
pub const SED_IMP: u8 = 0xF8;

// INC
pub const INC_ZPG: u8 = 0xE6;
pub const INC_ZPX: u8 = 0xF6;
pub const INC_ABS: u8 = 0xEE;
pub const INC_ABX: u8 = 0xFE;

// JMP
pub const JMP_ABS: u8 = 0x4C;
pub const JMP_IND: u8 = 0x6C;

// JSR
pub const JSR_ABS: u8 = 0x20;

// LDA
pub const LDA_IMM: u8 = 0xA9;
pub const LDA_ZPG: u8 = 0xA5;
pub const LDA_ZPX: u8 = 0xB5;
pub const LDA_ABS: u8 = 0xAD;
pub const LDA_ABX: u8 = 0xBD;
pub const LDA_ABY: u8 = 0xB9;
pub const LDA_INX: u8 = 0xA1;
pub const LDA_INY: u8 = 0xB1;

// LDX
pub const LDX_IMM: u8 = 0xA2;
pub const LDX_ZPG: u8 = 0xA6;
pub const LDX_ZPY: u8 = 0xB6;
pub const LDX_ABS: u8 = 0xAE;
pub const LDX_ABY: u8 = 0xBE;

// LDY
pub const LDY_IMM: u8 = 0xA0;
pub const LDY_ZPG: u8 = 0xA4;
pub const LDY_ZPX: u8 = 0xB4;
pub const LDY_ABS: u8 = 0xAC;
pub const LDY_ABX: u8 = 0xBC;

// LSR
pub const LSR_ACC: u8 = 0x4A;
pub const LSR_ZPG: u8 = 0x46;
pub const LSR_ZPX: u8 = 0x56;
pub const LSR_ABS: u8 = 0x4E;
pub const LSR_ABX: u8 = 0x5E;

// NOP
pub const NOP_IMP: u8 = 0xEA;

// ORA
pub const ORA_IMM: u8 = 0x09;
pub const ORA_ZPG: u8 = 0x05;
pub const ORA_ZPX: u8 = 0x15;
pub const ORA_ABS: u8 = 0x0D;
pub const ORA_ABX: u8 = 0x1D;
pub const ORA_ABY: u8 = 0x19;
pub const ORA_INX: u8 = 0x01;
pub const ORA_INY: u8 = 0x11;

// Register Instructions
pub const TAX_IMP: u8 = 0xAA;
pub const TXA_IMP: u8 = 0x8A;
pub const DEX_IMP: u8 = 0xCA;
pub const INX_IMP: u8 = 0xE8;
pub const TAY_IMP: u8 = 0xA8;
pub const TYA_IMP: u8 = 0x98;
pub const DEY_IMP: u8 = 0x88;
pub const INY_IMP: u8 = 0xC8;

// ROL
pub const ROL_ACC: u8 = 0x2A;
pub const ROL_ZPG: u8 = 0x26;
pub const ROL_ZPX: u8 = 0x36;
pub const ROL_ABS: u8 = 0x2E;
pub const ROL_ABX: u8 = 0x3E;

// ROR
pub const ROR_ACC: u8 = 0x6A;
pub const ROR_ZPG: u8 = 0x66;
pub const ROR_ZPX: u8 = 0x76;
pub const ROR_ABS: u8 = 0x6E;
pub const ROR_ABX: u8 = 0x7E;

// RTI
pub const RTI_IMP: u8 = 0x40;

// RTS
pub const RTS_IMP: u8 = 0x60;

// SBC
pub const SBC_IMM: u8 = 0xE9;
pub const SBC_ZPG: u8 = 0xE5;
pub const SBC_ZPX: u8 = 0xF5;
pub const SBC_ABS: u8 = 0xED;
pub const SBC_ABX: u8 = 0xFD;
pub const SBC_ABY: u8 = 0xF9;
pub const SBC_INX: u8 = 0xE1;
pub const SBC_INY: u8 = 0xF1;

// STA
pub const STA_ZPG: u8 = 0x85;
pub const STA_ZPX: u8 = 0x95;
pub const STA_ABS: u8 = 0x8D;
pub const STA_ABX: u8 = 0x9D;
pub const STA_ABY: u8 = 0x99;
pub const STA_INX: u8 = 0x81;
pub const STA_INY: u8 = 0x91;

// Stack Instructions
pub const TXS_IMP: u8 = 0x9A;
pub const TSX_IMP: u8 = 0xBA;
pub const PHA_IMP: u8 = 0x48;
pub const PLA_IMP: u8 = 0x68;
pub const PHP_IMP: u8 = 0x08;
pub const PLP_IMP: u8 = 0x28;

// STX
pub const STX_ZPG: u8 = 0x86;
pub const STX_ZPY: u8 = 0x96;
pub const STX_ABS: u8 = 0x8E;

// STY
pub const STY_ZPG: u8 = 0x84;
pub const STY_ZPX: u8 = 0x94;
pub const STY_ABS: u8 = 0x8C;