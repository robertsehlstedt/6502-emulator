use crate::addressmode::AddrMode;
use crate::operation::OpCode;
use crate::instruction_hex::*;

#[derive(Debug)]
pub struct Instruction {
    pub op: OpCode,
    pub mode: AddrMode,
}

// @Improvement: Instead of using a match, I could create a table with all instructions..
// const X: &[Option<Instruction>; u8::MAX as usize] = [
//
// ];

pub fn decode_instruction(instr: u8) -> Option<Instruction> {
    match instr {
        // ADC
        ADC_IMM => Some(Instruction{op: OpCode::ADC, mode: AddrMode::IMM}),
        ADC_ZPG => Some(Instruction{op: OpCode::ADC, mode: AddrMode::ZPG}),
        ADC_ZPX => Some(Instruction{op: OpCode::ADC, mode: AddrMode::ZPX}),
        ADC_ABS => Some(Instruction{op: OpCode::ADC, mode: AddrMode::ABS}),
        ADC_ABX => Some(Instruction{op: OpCode::ADC, mode: AddrMode::ABX}),
        ADC_ABY => Some(Instruction{op: OpCode::ADC, mode: AddrMode::ABY}),
        ADC_INX => Some(Instruction{op: OpCode::ADC, mode: AddrMode::INX}),
        ADC_INY => Some(Instruction{op: OpCode::ADC, mode: AddrMode::INY}),

        // AND
        AND_IMM => Some(Instruction{op: OpCode::AND, mode: AddrMode::IMM}),
        AND_ZPG => Some(Instruction{op: OpCode::AND, mode: AddrMode::ZPG}),
        AND_ZPX => Some(Instruction{op: OpCode::AND, mode: AddrMode::ZPX}),
        AND_ABS => Some(Instruction{op: OpCode::AND, mode: AddrMode::ABS}),
        AND_ABX => Some(Instruction{op: OpCode::AND, mode: AddrMode::ABX}),
        AND_ABY => Some(Instruction{op: OpCode::AND, mode: AddrMode::ABY}),
        AND_INX => Some(Instruction{op: OpCode::AND, mode: AddrMode::INX}),
        AND_INY => Some(Instruction{op: OpCode::AND, mode: AddrMode::INY}),

        // ASL
        ASL_ACC => Some(Instruction{op: OpCode::ASL, mode: AddrMode::ACC}),
        ASL_ZPG => Some(Instruction{op: OpCode::ASL, mode: AddrMode::ZPG}),
        ASL_ZPX => Some(Instruction{op: OpCode::ASL, mode: AddrMode::ZPX}),
        ASL_ABS => Some(Instruction{op: OpCode::ASL, mode: AddrMode::ABS}),
        ASL_ABX => Some(Instruction{op: OpCode::ASL, mode: AddrMode::ABX}),

        // BIT
        BIT_ZPG => Some(Instruction{op: OpCode::BIT, mode: AddrMode::ZPG}),
        BIT_ABS => Some(Instruction{op: OpCode::BIT, mode: AddrMode::ABS}),

        // Branch Instructions
        BPL_REL => Some(Instruction{op: OpCode::BPL, mode: AddrMode::REL}),
        BMI_REL => Some(Instruction{op: OpCode::BMI, mode: AddrMode::REL}),
        BVC_REL => Some(Instruction{op: OpCode::BVC, mode: AddrMode::REL}),
        BVS_REL => Some(Instruction{op: OpCode::BVS, mode: AddrMode::REL}),
        BCC_REL => Some(Instruction{op: OpCode::BCC, mode: AddrMode::REL}),
        BCS_REL => Some(Instruction{op: OpCode::BCS, mode: AddrMode::REL}),
        BNE_REL => Some(Instruction{op: OpCode::BNE, mode: AddrMode::REL}),
        BEQ_REL => Some(Instruction{op: OpCode::BEQ, mode: AddrMode::REL}),

        // BRK
        BRK_IMP => Some(Instruction{op: OpCode::BRK, mode: AddrMode::IMP}),

        // CMP
        CMP_IMM => Some(Instruction{op: OpCode::CMP, mode: AddrMode::IMM}),
        CMP_ZPG => Some(Instruction{op: OpCode::CMP, mode: AddrMode::ZPG}),
        CMP_ZPX => Some(Instruction{op: OpCode::CMP, mode: AddrMode::ZPX}),
        CMP_ABS => Some(Instruction{op: OpCode::CMP, mode: AddrMode::ABS}),
        CMP_ABX => Some(Instruction{op: OpCode::CMP, mode: AddrMode::ABX}),
        CMP_ABY => Some(Instruction{op: OpCode::CMP, mode: AddrMode::ABY}),
        CMP_INX => Some(Instruction{op: OpCode::CMP, mode: AddrMode::INX}),
        CMP_INY => Some(Instruction{op: OpCode::CMP, mode: AddrMode::INY}),

        // CPX
        CPX_IMM => Some(Instruction{op: OpCode::CPX, mode: AddrMode::IMM}),
        CPX_ZPG => Some(Instruction{op: OpCode::CPX, mode: AddrMode::ZPG}),
        CPX_ABS => Some(Instruction{op: OpCode::CPX, mode: AddrMode::ABS}),
        
        // CPY
        CPY_IMM => Some(Instruction{op: OpCode::CPY, mode: AddrMode::IMM}),
        CPY_ZPG => Some(Instruction{op: OpCode::CPY, mode: AddrMode::ZPG}),
        CPY_ABS => Some(Instruction{op: OpCode::CPY, mode: AddrMode::ABS}),
        
        // DEC
        DEC_ZPG => Some(Instruction{op: OpCode::DEC, mode: AddrMode::ZPG}),
        DEC_ZPX => Some(Instruction{op: OpCode::DEC, mode: AddrMode::ZPX}),
        DEC_ABS => Some(Instruction{op: OpCode::DEC, mode: AddrMode::ABS}),
        DEC_ABX => Some(Instruction{op: OpCode::DEC, mode: AddrMode::ABX}),

        // EOR
        EOR_IMM => Some(Instruction{op: OpCode::EOR, mode: AddrMode::IMM}),
        EOR_ZPG => Some(Instruction{op: OpCode::EOR, mode: AddrMode::ZPG}),
        EOR_ZPX => Some(Instruction{op: OpCode::EOR, mode: AddrMode::ZPX}),
        EOR_ABS => Some(Instruction{op: OpCode::EOR, mode: AddrMode::ABS}),
        EOR_ABX => Some(Instruction{op: OpCode::EOR, mode: AddrMode::ABX}),
        EOR_ABY => Some(Instruction{op: OpCode::EOR, mode: AddrMode::ABY}),
        EOR_INX => Some(Instruction{op: OpCode::EOR, mode: AddrMode::INX}),
        EOR_INY => Some(Instruction{op: OpCode::EOR, mode: AddrMode::INY}),

        // Flag Instructions
        CLC_IMP => Some(Instruction{op: OpCode::CLC, mode: AddrMode::IMP}),
        SEC_IMP => Some(Instruction{op: OpCode::SEC, mode: AddrMode::IMP}),
        CLI_IMP => Some(Instruction{op: OpCode::CLI, mode: AddrMode::IMP}),
        SEI_IMP => Some(Instruction{op: OpCode::SEI, mode: AddrMode::IMP}),
        CLV_IMP => Some(Instruction{op: OpCode::CLV, mode: AddrMode::IMP}),
        CLD_IMP => Some(Instruction{op: OpCode::CLD, mode: AddrMode::IMP}),
        SED_IMP => Some(Instruction{op: OpCode::SED, mode: AddrMode::IMP}),

        // INC
        INC_ZPG => Some(Instruction{op: OpCode::INC, mode: AddrMode::ZPG}),
        INC_ZPX => Some(Instruction{op: OpCode::INC, mode: AddrMode::ZPX}),
        INC_ABS => Some(Instruction{op: OpCode::INC, mode: AddrMode::ABS}),
        INC_ABX => Some(Instruction{op: OpCode::INC, mode: AddrMode::ABX}),

        // JMP
        JMP_ABS => Some(Instruction{op: OpCode::JMP, mode: AddrMode::ABS}),
        JMP_IND => Some(Instruction{op: OpCode::JMP, mode: AddrMode::IND}),

        // JSR
        JSR_ABS => Some(Instruction{op: OpCode::JSR, mode: AddrMode::ABS}),

        // LDA
        LDA_IMM => Some(Instruction{op: OpCode::LDA, mode: AddrMode::IMM}),
        LDA_ZPG => Some(Instruction{op: OpCode::LDA, mode: AddrMode::ZPG}),
        LDA_ZPX => Some(Instruction{op: OpCode::LDA, mode: AddrMode::ZPX}),
        LDA_ABS => Some(Instruction{op: OpCode::LDA, mode: AddrMode::ABS}),
        LDA_ABX => Some(Instruction{op: OpCode::LDA, mode: AddrMode::ABX}),
        LDA_ABY => Some(Instruction{op: OpCode::LDA, mode: AddrMode::ABY}),
        LDA_INX => Some(Instruction{op: OpCode::LDA, mode: AddrMode::INX}),
        LDA_INY => Some(Instruction{op: OpCode::LDA, mode: AddrMode::INY}),

        // LDX
        LDX_IMM => Some(Instruction{op: OpCode::LDX, mode: AddrMode::IMM}),
        LDX_ZPG => Some(Instruction{op: OpCode::LDX, mode: AddrMode::ZPG}),
        LDX_ZPY => Some(Instruction{op: OpCode::LDX, mode: AddrMode::ZPY}),
        LDX_ABS => Some(Instruction{op: OpCode::LDX, mode: AddrMode::ABS}),
        LDX_ABY => Some(Instruction{op: OpCode::LDX, mode: AddrMode::ABY}),

        // LDY
        LDY_IMM => Some(Instruction{op: OpCode::LDY, mode: AddrMode::IMM}),
        LDY_ZPG => Some(Instruction{op: OpCode::LDY, mode: AddrMode::ZPG}),
        LDY_ZPX => Some(Instruction{op: OpCode::LDY, mode: AddrMode::ZPX}),
        LDY_ABS => Some(Instruction{op: OpCode::LDY, mode: AddrMode::ABS}),
        LDY_ABX => Some(Instruction{op: OpCode::LDY, mode: AddrMode::ABX}),

        // LSR
        LSR_ACC => Some(Instruction{op: OpCode::LSR, mode: AddrMode::ACC}),
        LSR_ZPG => Some(Instruction{op: OpCode::LSR, mode: AddrMode::ZPG}),
        LSR_ZPX => Some(Instruction{op: OpCode::LSR, mode: AddrMode::ZPX}),
        LSR_ABS => Some(Instruction{op: OpCode::LSR, mode: AddrMode::ABS}),
        LSR_ABX => Some(Instruction{op: OpCode::LSR, mode: AddrMode::ABX}),

        // NOP
        NOP_IMP => Some(Instruction{op: OpCode::NOP, mode: AddrMode::IMP}),

        // ORA
        ORA_IMM => Some(Instruction{op: OpCode::ORA, mode: AddrMode::IMM}),
        ORA_ZPG => Some(Instruction{op: OpCode::ORA, mode: AddrMode::ZPG}),
        ORA_ZPX => Some(Instruction{op: OpCode::ORA, mode: AddrMode::ZPX}),
        ORA_ABS => Some(Instruction{op: OpCode::ORA, mode: AddrMode::ABS}),
        ORA_ABX => Some(Instruction{op: OpCode::ORA, mode: AddrMode::ABX}),
        ORA_ABY => Some(Instruction{op: OpCode::ORA, mode: AddrMode::ABY}),
        ORA_INX => Some(Instruction{op: OpCode::ORA, mode: AddrMode::INX}),
        ORA_INY => Some(Instruction{op: OpCode::ORA, mode: AddrMode::INY}),

        // Register Instructions
        TAX_IMP => Some(Instruction{op: OpCode::TAX, mode: AddrMode::IMP}),
        TXA_IMP => Some(Instruction{op: OpCode::TXA, mode: AddrMode::IMP}),
        DEX_IMP => Some(Instruction{op: OpCode::DEX, mode: AddrMode::IMP}),
        INX_IMP => Some(Instruction{op: OpCode::INX, mode: AddrMode::IMP}),
        TAY_IMP => Some(Instruction{op: OpCode::TAY, mode: AddrMode::IMP}),
        TYA_IMP => Some(Instruction{op: OpCode::TYA, mode: AddrMode::IMP}),
        DEY_IMP => Some(Instruction{op: OpCode::DEY, mode: AddrMode::IMP}),
        INY_IMP => Some(Instruction{op: OpCode::INY, mode: AddrMode::IMP}),

        // ROL
        ROL_ACC => Some(Instruction{op: OpCode::ROL, mode: AddrMode::ACC}),
        ROL_ZPG => Some(Instruction{op: OpCode::ROL, mode: AddrMode::ZPG}),
        ROL_ZPX => Some(Instruction{op: OpCode::ROL, mode: AddrMode::ZPX}),
        ROL_ABS => Some(Instruction{op: OpCode::ROL, mode: AddrMode::ABS}),
        ROL_ABX => Some(Instruction{op: OpCode::ROL, mode: AddrMode::ABX}),

        // ROR
        ROR_ACC => Some(Instruction{op: OpCode::ROR, mode: AddrMode::ACC}),
        ROR_ZPG => Some(Instruction{op: OpCode::ROR, mode: AddrMode::ZPG}),
        ROR_ZPX => Some(Instruction{op: OpCode::ROR, mode: AddrMode::ZPX}),
        ROR_ABS => Some(Instruction{op: OpCode::ROR, mode: AddrMode::ABS}),
        ROR_ABX => Some(Instruction{op: OpCode::ROR, mode: AddrMode::ABX}),

        // RTI
        RTI_IMP => Some(Instruction{op: OpCode::RTI, mode: AddrMode::IMP}),

        // RTS
        RTS_IMP => Some(Instruction{op: OpCode::RTS, mode: AddrMode::IMP}),

        // SBC
        SBC_IMM => Some(Instruction{op: OpCode::SBC, mode: AddrMode::IMM}),
        SBC_ZPG => Some(Instruction{op: OpCode::SBC, mode: AddrMode::ZPG}),
        SBC_ZPX => Some(Instruction{op: OpCode::SBC, mode: AddrMode::ZPX}),
        SBC_ABS => Some(Instruction{op: OpCode::SBC, mode: AddrMode::ABS}),
        SBC_ABX => Some(Instruction{op: OpCode::SBC, mode: AddrMode::ABX}),
        SBC_ABY => Some(Instruction{op: OpCode::SBC, mode: AddrMode::ABY}),
        SBC_INX => Some(Instruction{op: OpCode::SBC, mode: AddrMode::INX}),
        SBC_INY => Some(Instruction{op: OpCode::SBC, mode: AddrMode::INY}),

        // STA
        STA_ZPG => Some(Instruction{op: OpCode::STA, mode: AddrMode::ZPG}),
        STA_ZPX => Some(Instruction{op: OpCode::STA, mode: AddrMode::ZPX}),
        STA_ABS => Some(Instruction{op: OpCode::STA, mode: AddrMode::ABS}),
        STA_ABX => Some(Instruction{op: OpCode::STA, mode: AddrMode::ABX}),
        STA_ABY => Some(Instruction{op: OpCode::STA, mode: AddrMode::ABY}),
        STA_INX => Some(Instruction{op: OpCode::STA, mode: AddrMode::INX}),
        STA_INY => Some(Instruction{op: OpCode::STA, mode: AddrMode::INY}),

        // Stack Instructions
        TXS_IMP => Some(Instruction{op: OpCode::TXS, mode: AddrMode::IMP}),
        TSX_IMP => Some(Instruction{op: OpCode::TSX, mode: AddrMode::IMP}),
        PHA_IMP => Some(Instruction{op: OpCode::PHA, mode: AddrMode::IMP}),
        PLA_IMP => Some(Instruction{op: OpCode::PLA, mode: AddrMode::IMP}),
        PHP_IMP => Some(Instruction{op: OpCode::PHP, mode: AddrMode::IMP}),
        PLP_IMP => Some(Instruction{op: OpCode::PLP, mode: AddrMode::IMP}),

        // STX
        STX_ZPG => Some(Instruction{op: OpCode::STX, mode: AddrMode::ZPG}),
        STX_ZPY => Some(Instruction{op: OpCode::STX, mode: AddrMode::ZPY}),
        STX_ABS => Some(Instruction{op: OpCode::STX, mode: AddrMode::ABS}),

        // STY
        STY_ZPG => Some(Instruction{op: OpCode::STY, mode: AddrMode::ZPG}),
        STY_ZPX => Some(Instruction{op: OpCode::STY, mode: AddrMode::ZPX}),
        STY_ABS => Some(Instruction{op: OpCode::STY, mode: AddrMode::ABS}),

        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adc_imm() {
        let instr = decode_instruction(ADC_IMM);
        assert!(instr.is_some());
        let instr = instr.unwrap();
        assert_eq!(instr.op, OpCode::ADC);
        assert_eq!(instr.mode, AddrMode::IMM);
    }

    #[test]
    fn test_adc_zpg() {
        let instr = decode_instruction(ADC_ZPG);
        assert!(instr.is_some());
        let instr = instr.unwrap();
        assert_eq!(instr.op, OpCode::ADC);
        assert_eq!(instr.mode, AddrMode::ZPG);
    }

    #[test]
    fn test_adc_zpx() {
        let instr = decode_instruction(ADC_ZPX);
        assert!(instr.is_some());
        let instr = instr.unwrap();
        assert_eq!(instr.op, OpCode::ADC);
        assert_eq!(instr.mode, AddrMode::ZPX);
    }

    #[test]
    fn test_adc_abs() {
        let instr = decode_instruction(ADC_ABS);
        assert!(instr.is_some());
        let instr = instr.unwrap();
        assert_eq!(instr.op, OpCode::ADC);
        assert_eq!(instr.mode, AddrMode::ABS);
    }

    #[test]
    fn test_adc_abx() {
        let instr = decode_instruction(ADC_ABX);
        assert!(instr.is_some());
        let instr = instr.unwrap();
        assert_eq!(instr.op, OpCode::ADC);
        assert_eq!(instr.mode, AddrMode::ABX);
    }

    #[test]
    fn test_adc_aby() {
        let instr = decode_instruction(ADC_ABY);
        assert!(instr.is_some());
        let instr = instr.unwrap();
        assert_eq!(instr.op, OpCode::ADC);
        assert_eq!(instr.mode, AddrMode::ABY);
    }

    #[test]
    fn test_adc_inx() {
        let instr = decode_instruction(ADC_INX);
        assert!(instr.is_some());
        let instr = instr.unwrap();
        assert_eq!(instr.op, OpCode::ADC);
        assert_eq!(instr.mode, AddrMode::INX);
    }

    #[test]
    fn test_adc_iny() {
        let instr = decode_instruction(ADC_INY);
        assert!(instr.is_some());
        let instr = instr.unwrap();
        assert_eq!(instr.op, OpCode::ADC);
        assert_eq!(instr.mode, AddrMode::INY);
    }
}