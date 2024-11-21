use crate::Cpu;
use crate::Bus;
use crate::Variant;
use crate::instruction::{AddressingMode, OperationInput, InstructionCode, Instruction};

const STACK_BASE:       u8 = 0x01;
const VECTOR_BASE:      u8 = 0xFF;
const IRQ_BRK_VECTOR:   u8 = 0xFE;
const RESET_VECTOR:     u8 = 0xFC;
const NMI_VECTOR:       u8 = 0xFA;

pub struct CpuWithBus<'c, B, V> {
    pub cpu: &'c mut Cpu<V>,
    pub bus: &'c mut B,
}

impl<B: Bus, V: Variant> CpuWithBus<'_, B, V> {
    pub fn reset(&mut self) {
        self.cpu.reg.i = true;
        self.cpu.sp = self.cpu.sp.wrapping_sub(3);
        self.cpu.pc = self.read_u16(VECTOR_BASE, RESET_VECTOR);
    }
    
    pub fn irq(&mut self) {
        if !self.cpu.reg.i {
            self.interrupt(IRQ_BRK_VECTOR, false);
        }
    }

    pub fn nmi(&mut self) {
        self.interrupt(NMI_VECTOR, false);
    }

    pub fn step(&mut self) {
        let (instr_code, addr_mode) = V::decode(self.take_u8_at_pc()).unwrap();
        let op_input = self.execute_addressing(addr_mode);
        self.execute_operation((instr_code, op_input));
    }

    fn read_u16(&mut self, high: u8, low: u8) -> u16 {
        let u16_low = u16::from_le_bytes([low, high]);
        let u16_high = u16::from_le_bytes([low.wrapping_add(1), high]);
        u16::from_le_bytes([self.bus.read(u16_low), self.bus.read(u16_high)])
    }

    fn take_u8_at_pc(&mut self) -> u8 {
        let byte = self.bus.read(self.cpu.pc);
        self.cpu.pc = self.cpu.pc.wrapping_add(1);
        byte
    }

    fn take_u16_at_pc(&mut self) -> u16 {
        u16::from_le_bytes([self.take_u8_at_pc(), self.take_u8_at_pc()])
    }

    fn stack_push(&mut self, value: u8) {
        let addr = u16::from_le_bytes([self.cpu.sp, STACK_BASE]);
        self.bus.write(addr, value);
        self.cpu.sp = self.cpu.sp.wrapping_sub(1);
    }

    fn stack_pop(&mut self) -> u8 {
        self.cpu.sp = self.cpu.sp.wrapping_add(1);
        let addr = u16::from_le_bytes([self.cpu.sp, STACK_BASE]);
        self.bus.read(addr)
    }

    fn interrupt(&mut self, vector: u8, brk: bool) {
        let [pc_low, pc_high] = self.cpu.pc.to_le_bytes();
        self.stack_push(pc_high);
        self.stack_push(pc_low);
        self.stack_push(self.cpu.reg.get_status(brk));
        self.cpu.reg.i = true;
        self.cpu.pc = self.read_u16(VECTOR_BASE, vector);
    }

    fn execute_addressing(&mut self, am: AddressingMode) -> OperationInput {
        match am {
            AddressingMode::ACC | AddressingMode::IMP => {
                OperationInput::IMP
            }
            AddressingMode::IMM => {
                let val = self.take_u8_at_pc();
                OperationInput::IMM(val)
            }
            AddressingMode::ZPG => {
                let addr = self.take_u8_at_pc() as u16;
                OperationInput::ADR(addr)
            }
            AddressingMode::ZPX => {
                let addr = self.take_u8_at_pc().wrapping_add(self.cpu.reg.get_x()) as u16;
                OperationInput::ADR(addr)
            }
            AddressingMode::ZPY => {
                let addr = self.take_u8_at_pc().wrapping_add(self.cpu.reg.get_y()) as u16;
                OperationInput::ADR(addr)
            }
            AddressingMode::REL => {
                let offset = self.take_u8_at_pc() as i8 as u16;
                OperationInput::REL(offset)
            }
            AddressingMode::ABS => {
                let addr = self.take_u16_at_pc();
                OperationInput::ADR(addr)
            }
            AddressingMode::ABX => {
                let addr = self.take_u16_at_pc().wrapping_add(self.cpu.reg.get_x() as u16);
                OperationInput::ADR(addr)
            }
            AddressingMode::ABY => {
                let addr = self.take_u16_at_pc().wrapping_add(self.cpu.reg.get_y() as u16);
                OperationInput::ADR(addr)
            }
            AddressingMode::IND => {
                let low = self.take_u8_at_pc();
                let high = self.take_u8_at_pc();
                let addr = self.read_u16(high, low);
                OperationInput::ADR(addr)
            }
            AddressingMode::INX => {
                let low = self.take_u8_at_pc().wrapping_add(self.cpu.reg.get_x());
                let addr = self.read_u16(0, low);
                OperationInput::ADR(addr)
            }
            AddressingMode::INY => {
                let low = self.take_u8_at_pc().wrapping_add(self.cpu.reg.get_y());
                let addr = self.read_u16(0, low);
                OperationInput::ADR(addr)
            }
        }
    }

    fn execute_operation(&mut self, instruction: Instruction) {
        match instruction  {
            (InstructionCode::ADC, OperationInput::IMM(val)) => self.adc_imm(val),
            (InstructionCode::ADC, OperationInput::ADR(addr)) => self.adc_adr(addr),

            (InstructionCode::AND, OperationInput::IMM(val)) => self.and_imm(val),
            (InstructionCode::AND, OperationInput::ADR(addr)) => self.and_adr(addr),

            (InstructionCode::ASL, OperationInput::IMP) => self.asl_imp(),
            (InstructionCode::ASL, OperationInput::ADR(addr)) => self.asl_adr(addr),

            (InstructionCode::BCC, OperationInput::REL(offset)) => self.bcc(offset),
            
            (InstructionCode::BCS, OperationInput::REL(offset)) => self.bcs(offset),

            (InstructionCode::BEQ, OperationInput::REL(offset)) => self.beq(offset),

            (InstructionCode::BIT, OperationInput::ADR(addr)) => self.bit(addr),

            (InstructionCode::BMI, OperationInput::REL(offset)) => self.bmi(offset),

            (InstructionCode::BNE, OperationInput::REL(offset)) => self.bne(offset),

            (InstructionCode::BPL, OperationInput::REL(offset)) => self.bpl(offset),

            (InstructionCode::BRK, OperationInput::IMP) => self.brk(),

            (InstructionCode::BVC, OperationInput::REL(offset)) => self.bvc(offset),

            (InstructionCode::BVS, OperationInput::REL(offset)) => self.bvs(offset),

            (InstructionCode::CLC, OperationInput::IMP) => self.clc(),

            (InstructionCode::CLD, OperationInput::IMP) => self.cld(),

            (InstructionCode::CLI, OperationInput::IMP) => self.cli(),

            (InstructionCode::CLV, OperationInput::IMP) => self.clv(),

            (InstructionCode::CMP, OperationInput::IMM(val)) => self.cmp_imm(val),
            (InstructionCode::CMP, OperationInput::ADR(addr)) => self.cmp_adr(addr),

            (InstructionCode::CPX, OperationInput::IMM(val)) => self.cpx_imm(val),
            (InstructionCode::CPX, OperationInput::ADR(addr)) => self.cpx_adr(addr),

            (InstructionCode::CPY, OperationInput::IMM(val)) => self.cpy_imm(val),
            (InstructionCode::CPY, OperationInput::ADR(addr)) => self.cpy_adr(addr),

            (InstructionCode::DEC, OperationInput::ADR(addr)) => self.dec(addr),

            (InstructionCode::DEX, OperationInput::IMP) => self.dex(),

            (InstructionCode::DEY, OperationInput::IMP) => self.dey(),

            (InstructionCode::EOR, OperationInput::IMM(val)) => self.eor_imm(val),
            (InstructionCode::EOR, OperationInput::ADR(addr)) => self.eor_adr(addr),

            (InstructionCode::INC, OperationInput::ADR(addr)) => self.inc(addr),

            (InstructionCode::INX, OperationInput::IMP) => self.inx(),

            (InstructionCode::INY, OperationInput::IMP) => self.iny(),

            (InstructionCode::JMP, OperationInput::ADR(addr)) => self.jmp(addr),

            (InstructionCode::JSR, OperationInput::ADR(addr)) => self.jsr(addr),

            (InstructionCode::LDA, OperationInput::IMM(val)) => self.lda_imm(val),
            (InstructionCode::LDA, OperationInput::ADR(addr)) => self.lda_adr(addr),

            (InstructionCode::LDX, OperationInput::IMM(val)) => self.ldx_imm(val),
            (InstructionCode::LDX, OperationInput::ADR(addr)) => self.ldx_adr(addr),

            (InstructionCode::LDY, OperationInput::IMM(val)) => self.ldy_imm(val),
            (InstructionCode::LDY, OperationInput::ADR(addr)) => self.ldy_adr(addr),

            (InstructionCode::LSR, OperationInput::IMP) => self.lsr_imp(),
            (InstructionCode::LSR, OperationInput::ADR(addr)) => self.lsr_adr(addr),

            (InstructionCode::NOP, OperationInput::IMP) => self.nop(),

            (InstructionCode::ORA, OperationInput::IMM(val)) => self.ora_imm(val),
            (InstructionCode::ORA, OperationInput::ADR(addr)) => self.ora_adr(addr),

            (InstructionCode::PHA, OperationInput::IMP) => self.pha(),

            (InstructionCode::PHP, OperationInput::IMP) => self.php(),

            (InstructionCode::PLA, OperationInput::IMP) => self.pla(),

            (InstructionCode::PLP, OperationInput::IMP) => self.plp(),

            (InstructionCode::ROL, OperationInput::IMP) => self.rol_imp(),
            (InstructionCode::ROL, OperationInput::ADR(addr)) => self.rol_adr(addr),

            (InstructionCode::ROR, OperationInput::IMP) => self.ror_imp(),
            (InstructionCode::ROR, OperationInput::ADR(addr)) => self.ror_adr(addr),

            (InstructionCode::RTI, OperationInput::IMP) => self.rti(),

            (InstructionCode::RTS, OperationInput::IMP) => self.rts(),

            (InstructionCode::SBC, OperationInput::IMM(val)) => todo!(),
            (InstructionCode::SBC, OperationInput::ADR(addr)) => todo!(),

            (InstructionCode::SEC, OperationInput::IMP) => self.sec(),

            (InstructionCode::SED, OperationInput::IMP) => self.sed(),

            (InstructionCode::SEI, OperationInput::IMP) => self.sei(),

            (InstructionCode::STA, OperationInput::ADR(addr)) => self.sta(addr),

            (InstructionCode::STX, OperationInput::ADR(addr)) => self.stx(addr),

            (InstructionCode::STY, OperationInput::ADR(addr)) => self.sty(addr),

            (InstructionCode::TAX, OperationInput::IMP) => self.tax(),

            (InstructionCode::TAY, OperationInput::IMP) => self.tay(),

            (InstructionCode::TSX, OperationInput::IMP) => self.tsx(),

            (InstructionCode::TXA, OperationInput::IMP) => self.txa(),

            (InstructionCode::TXS, OperationInput::IMP) => self.txs(),

            (InstructionCode::TYA, OperationInput::IMP) => self.tya(),

            _illegal => panic!(),
        }
    }

    fn adc_imm(&mut self, value: u8) {
        let value = value as u16;
        let carry = self.cpu.reg.c as u16;
        let result = self.cpu.reg.get_a() as u16 + value + carry;
        let seven_bit_result = (self.cpu.reg.get_a() as u16 & 0x7F) + (value & 0x7F) + carry;
        let carry_out = result > 0xFF;
        let seven_bit_carry_out = seven_bit_result > 0x7F;
        self.cpu.reg.c = carry_out;
        self.cpu.reg.v = carry_out != seven_bit_carry_out;
        self.cpu.reg.update_a(result as u8);
    }

    fn adc_adr(&mut self, addr: u16) {
        let value = self.bus.read(addr);
        self.adc_imm(value);
    }

    fn and_imm(&mut self, value: u8) {
        self.cpu.reg.update_a(self.cpu.reg.get_a() & value);
    }

    fn and_adr(&mut self, addr: u16) {
        let value = self.bus.read(addr);
        self.and_imm(value);
    }
    
    fn asl_imp(&mut self) {
        self.cpu.reg.c = self.cpu.reg.get_a() & 0b1000_0000 != 0;
        self.cpu.reg.update_a(self.cpu.reg.get_a() << 1);
    }

    fn asl_adr(&mut self, addr: u16) {
        let n = self.bus.read(addr);
        self.cpu.reg.c = n & 0b1000_0000 != 0;
        let result = n << 1;
        self.bus.write(addr, result);
        self.cpu.reg.update_nz_flags(result);
    }
    
    fn bcc(&mut self, offset: u16) {
        if !self.cpu.reg.c {
            let addr = self.cpu.pc.wrapping_add(offset);
            self.cpu.pc = addr;
        }
    }

    fn bcs(&mut self, offset: u16) {
        if self.cpu.reg.c {
            let addr = self.cpu.pc.wrapping_add(offset);
            self.cpu.pc = addr;
        }
    }

    fn beq(&mut self, offset: u16) {
        if self.cpu.reg.z {
            let addr = self.cpu.pc.wrapping_add(offset);
            self.cpu.pc = addr;
        }
    }

    fn bit(&mut self, addr: u16) {
        let n = self.bus.read(addr);
        self.cpu.reg.z = self.cpu.reg.get_a() & n == 0;
        self.cpu.reg.v = n & 0b0100_0000 != 0;
        self.cpu.reg.n = n & 0b1000_0000 != 0;
    }

    fn bmi(&mut self, offset: u16) {
        if self.cpu.reg.n {
            let addr = self.cpu.pc.wrapping_add(offset);
            self.cpu.pc = addr;
        }
    }

    fn bne(&mut self, offset: u16) {
        if !self.cpu.reg.z {
            let addr = self.cpu.pc.wrapping_add(offset);
            self.cpu.pc = addr;
        }
    }

    fn bpl(&mut self, offset: u16) {
        if !self.cpu.reg.n {
            let addr = self.cpu.pc.wrapping_add(offset);
            self.cpu.pc = addr;
        }
    }

    fn brk(&mut self) {
        self.cpu.pc = self.cpu.pc.wrapping_add(1);
        self.interrupt(IRQ_BRK_VECTOR, true);
    }

    fn bvc(&mut self, offset: u16) {
        if !self.cpu.reg.v {
            let addr = self.cpu.pc.wrapping_add(offset);
            self.cpu.pc = addr;
        }
    }

    fn bvs(&mut self, offset: u16) {
        if self.cpu.reg.v {
            let addr = self.cpu.pc.wrapping_add(offset);
            self.cpu.pc = addr;
        }
    }

    fn clc(&mut self) {
        self.cpu.reg.c = false;
    }

    fn cld(&mut self) {
        self.cpu.reg.d = false;
    }

    fn cli(&mut self) {
        self.cpu.reg.i = false;
    }

    fn clv(&mut self) {
        self.cpu.reg.v = false;
    }

    fn cmp_imm(&mut self, value: u8) {
        self.cpu.reg.update_nz_flags(self.cpu.reg.get_a().wrapping_sub(value));
        self.cpu.reg.c = self.cpu.reg.get_a() >= value;
    }

    fn cmp_adr(&mut self, addr: u16) {
        let value = self.bus.read(addr);
        self.cmp_imm(value);
    }

    fn cpx_imm(&mut self, value: u8) {
        self.cpu.reg.update_nz_flags(self.cpu.reg.get_x().wrapping_sub(value));
        self.cpu.reg.c = self.cpu.reg.get_x() >= value;
    }

    fn cpx_adr(&mut self, addr: u16) {
        let value = self.bus.read(addr);
        self.cpx_imm(value);
    }

    fn cpy_imm(&mut self, value: u8) {
        self.cpu.reg.update_nz_flags(self.cpu.reg.get_y().wrapping_sub(value));
        self.cpu.reg.c = self.cpu.reg.get_y() >= value;
    }

    fn cpy_adr(&mut self, addr: u16) {
        let value = self.bus.read(addr);
        self.cpy_imm(value);
    }
    
    fn dec(&mut self, addr: u16) {
        let n = self.bus.read(addr);
        let result = n.wrapping_sub(1);
        self.bus.write(addr, result);
        self.cpu.reg.update_nz_flags(result);
    }

    fn dex(&mut self) {
        self.cpu.reg.update_x(self.cpu.reg.get_x().wrapping_sub(1));
    }

    fn dey(&mut self) {
        self.cpu.reg.update_y(self.cpu.reg.get_y().wrapping_sub(1));
    }

    fn eor_imm(&mut self, value: u8) {
        self.cpu.reg.update_a(self.cpu.reg.get_a() ^ value);
    }

    fn eor_adr(&mut self, addr: u16) {
        let value = self.bus.read(addr);
        self.eor_imm(value);
    }

    fn inc(&mut self, addr: u16) {
        let n = self.bus.read(addr);
        let result = n.wrapping_add(1);
        self.bus.write(addr, result);
        self.cpu.reg.update_nz_flags(result);
    }

    fn inx(&mut self) {
        self.cpu.reg.update_x(self.cpu.reg.get_x().wrapping_add(1));
    }

    fn iny(&mut self) {
        self.cpu.reg.update_y(self.cpu.reg.get_y().wrapping_add(1));
    }

    fn jmp(&mut self, addr: u16) {
        self.cpu.pc = addr;
    }

    fn jsr(&mut self, addr: u16) {
        let return_addr = self.cpu.pc.wrapping_sub(1);
        let [ret_low, ret_high] = return_addr.to_le_bytes();
        self.stack_push(ret_high);
        self.stack_push(ret_low);
        self.cpu.pc = addr;
    }

    fn lda_imm(&mut self, value: u8) {
        self.cpu.reg.update_a(value);
    }

    fn lda_adr(&mut self, addr: u16) {
        let value = self.bus.read(addr);
        self.lda_imm(value);
    }

    fn ldx_imm(&mut self, value: u8) {
        self.cpu.reg.update_x(value);
    }

    fn ldx_adr(&mut self, addr: u16) {
        let value = self.bus.read(addr);
        self.ldx_imm(value);
    }

    fn ldy_imm(&mut self, value: u8) {
        self.cpu.reg.update_y(value);
    }

    fn ldy_adr(&mut self, addr: u16) {
        let value = self.bus.read(addr);
        self.ldy_imm(value);
    }

    fn lsr_imp(&mut self) {
        self.cpu.reg.c = self.cpu.reg.get_a() & 0b0000_0001 != 0;
        self.cpu.reg.update_a(self.cpu.reg.get_a() >> 1);
    }

    fn lsr_adr(&mut self, addr: u16) {
        let n = self.bus.read(addr);
        self.cpu.reg.c = n & 0b0000_0001 != 0;
        let result = n >> 1;
        self.bus.write(addr, result);
        self.cpu.reg.update_nz_flags(result);
    }

    fn nop(&self) { }

    fn ora_imm(&mut self, value: u8) {
        self.cpu.reg.update_a(self.cpu.reg.get_a() | value);
    }

    fn ora_adr(&mut self, addr: u16) {
        let value = self.bus.read(addr);
        self.ora_imm(value);
    }

    fn pha(&mut self) {
        self.stack_push(self.cpu.reg.get_a());
    }

    fn php(&mut self) {
        self.stack_push(self.cpu.reg.get_status(false));
    }

    fn pla(&mut self) {
        let value = self.stack_pop();
        self.cpu.reg.update_a(value);
    }

    fn plp(&mut self) {
        let status = self.stack_pop();
        self.cpu.reg.set_status(status);
    }

    fn rol_imp(&mut self) {
        let carry = self.cpu.reg.c;
        self.cpu.reg.c = self.cpu.reg.get_a() & 0b1000_0000 != 0;
        self.cpu.reg.update_a( (self.cpu.reg.get_a() << 1) | carry as u8);
    }

    fn rol_adr(&mut self, addr: u16) {
        let n = self.bus.read(addr);
        let carry = self.cpu.reg.c;
        self.cpu.reg.c = n & 0b1000_0000 != 0;
        let result = (n << 1) | carry as u8;
        self.bus.write(addr, result);
        self.cpu.reg.update_nz_flags(result);
    }

    fn ror_imp(&mut self) {
        let carry = self.cpu.reg.c;
        self.cpu.reg.c = self.cpu.reg.get_a() & 0b0000_0001 != 0;
        self.cpu.reg.update_a( (self.cpu.reg.get_a() >> 1) | ((carry as u8) << 7));
    }

    fn ror_adr(&mut self, addr: u16) {
        let n = self.bus.read(addr);
        let carry = self.cpu.reg.c;
        self.cpu.reg.c = n & 0b0000_0001 != 0;
        let result = (n >> 1) | ((carry as u8) << 7);
        self.bus.write(addr, result);
        self.cpu.reg.update_nz_flags(result);
    }

    fn rti(&mut self) {
        let status = self.stack_pop();
        self.cpu.reg.set_status(status);
        let pc_low = self.stack_pop();
        let pc_high = self.stack_pop();
        self.cpu.pc = u16::from_le_bytes([pc_low, pc_high]);
    }

    fn rts(&mut self) {
        let pc_low = self.stack_pop();
        let pc_high = self.stack_pop();
        self.cpu.pc = u16::from_le_bytes([pc_low, pc_high]).wrapping_add(1);
    }

    fn sec(&mut self) {
        self.cpu.reg.c = true;
    }

    fn sed(&mut self) {
        self.cpu.reg.d = true;
    }

    fn sei(&mut self) {
        self.cpu.reg.i = true;
    }

    fn sta(&mut self, addr: u16)  {
        self.bus.write(addr, self.cpu.reg.get_a());
    }

    fn stx(&mut self, addr: u16) {
        self.bus.write(addr, self.cpu.reg.get_x());
    }

    fn sty(&mut self, addr: u16) {
        self.bus.write(addr, self.cpu.reg.get_y());
    }

    fn tax(&mut self) {
        self.cpu.reg.update_x(self.cpu.reg.get_a());
    }

    fn tay(&mut self) {
        self.cpu.reg.update_y(self.cpu.reg.get_a());
    }

    fn tsx(&mut self) {
        self.cpu.reg.update_x(self.cpu.sp);
    }

    fn txa(&mut self) {
        self.cpu.reg.update_a(self.cpu.reg.get_x());
    }

    fn txs(&mut self) {
        self.cpu.sp = self.cpu.reg.get_x();
    }

    fn tya(&mut self) {
        self.cpu.reg.update_a(self.cpu.reg.get_y());
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockBus([u8; 65536]);
    impl Bus for MockBus {
        fn read(&self, addr: u16) -> u8 { self.0[addr as usize] }
        fn write(&mut self, addr: u16, value: u8) { self.0[addr as usize] = value }
    }

    struct MockVariant;
    impl Variant for MockVariant {
        fn decode(_: u8) -> Option<(
            crate::instruction::InstructionCode,
            crate::instruction::AddressingMode
        )> {
            None
        }
    }

    fn get_cpu() -> CpuWithBus<'static, MockBus, MockVariant> {
        let cpu = Box::leak(Box::new(Cpu::new(MockVariant)));
        let bus = Box::leak(Box::new(MockBus([0; 65536])));
        CpuWithBus {cpu: cpu, bus: bus}
    }

    #[test]
    fn test_and_imm() {
        let mut cwb = get_cpu();
        
        cwb.cpu.reg.update_a(0xFF);
        cwb.and_imm(0);
        assert_eq!(cwb.cpu.reg.get_a(), 0x00);
        assert!(cwb.cpu.reg.z);
        assert!(!cwb.cpu.reg.n);

        cwb.cpu.reg.update_a(0xFF);
        cwb.and_imm(0x01);
        assert_eq!(cwb.cpu.reg.get_a(), 0x01);
        assert!(!cwb.cpu.reg.z);
        assert!(!cwb.cpu.reg.n);

        cwb.cpu.reg.update_a(0xFF);
        cwb.and_imm(0b1000_0000);
        assert_eq!(cwb.cpu.reg.get_a(), 0b1000_0000);
        assert!(!cwb.cpu.reg.z);
        assert!(cwb.cpu.reg.n);
    }

    #[test]
    fn test_and_adr() {
        let mut cwb = get_cpu();
        
        cwb.cpu.reg.update_a(0xFF);
        cwb.bus.write(0, 0);
        cwb.and_adr(0);
        assert_eq!(cwb.cpu.reg.get_a(), 0x00);
        assert!(cwb.cpu.reg.z);
        assert!(!cwb.cpu.reg.n);

        cwb.cpu.reg.update_a(0xFF);
        cwb.bus.write(0, 0x01);
        cwb.and_adr(0);
        assert_eq!(cwb.cpu.reg.get_a(), 0x01);
        assert!(!cwb.cpu.reg.z);
        assert!(!cwb.cpu.reg.n);

        cwb.cpu.reg.update_a(0xFF);
        cwb.bus.write(0, 0b1000_0000);
        cwb.and_adr(0);
        assert_eq!(cwb.cpu.reg.get_a(), 0b1000_0000);
        assert!(!cwb.cpu.reg.z);
        assert!(cwb.cpu.reg.n);
    }

    #[test]
    fn test_asl_imp() {
        let mut cwb = get_cpu();

        cwb.cpu.reg.update_a(0b0000_0001);
        cwb.asl_imp();
        assert_eq!(cwb.cpu.reg.get_a(), 0b0000_0010);
        assert!(!cwb.cpu.reg.c);
        assert!(!cwb.cpu.reg.z);
        assert!(!cwb.cpu.reg.n);

        cwb.cpu.reg.update_a(0b1000_0000);
        cwb.asl_imp();
        assert_eq!(cwb.cpu.reg.get_a(), 0b0000_0000);
        assert!(cwb.cpu.reg.c);
        assert!(cwb.cpu.reg.z);
        assert!(!cwb.cpu.reg.n);

        cwb.cpu.reg.update_a(0b0100_0000);
        cwb.asl_imp();
        assert_eq!(cwb.cpu.reg.get_a(), 0b1000_0000);
        assert!(!cwb.cpu.reg.c);
        assert!(!cwb.cpu.reg.z);
        assert!(cwb.cpu.reg.n);
    }

    #[test]
    fn test_asl_adr() {
        let mut cwb = get_cpu();

        cwb.bus.write(0, 0b0000_0001);
        cwb.asl_adr(0);
        assert_eq!(cwb.bus.read(0), 0b0000_0010);
        assert!(!cwb.cpu.reg.c);
        assert!(!cwb.cpu.reg.z);
        assert!(!cwb.cpu.reg.n);

        cwb.bus.write(0, 0b1000_0000);
        cwb.asl_adr(0);
        assert_eq!(cwb.bus.read(0), 0b0000_0000);
        assert!(cwb.cpu.reg.c);
        assert!(cwb.cpu.reg.z);
        assert!(!cwb.cpu.reg.n);

        cwb.bus.write(0, 0b0100_0000);
        cwb.asl_adr(0);
        assert_eq!(cwb.bus.read(0), 0b1000_0000);
        assert!(!cwb.cpu.reg.c);
        assert!(!cwb.cpu.reg.z);
        assert!(cwb.cpu.reg.n);
    }

    #[test]
    fn test_bcc() {
        let mut cwb = get_cpu();
        cwb.cpu.reg.c = false;
        let before = cwb.cpu.pc;
        cwb.bcc(1);
        assert_eq!(cwb.cpu.pc, before.wrapping_add(1));
    }

    #[test]
    fn test_bcs() {
        let mut cwb = get_cpu();
        cwb.cpu.reg.c = true;
        let before = cwb.cpu.pc;
        cwb.bcs(1);
        assert_eq!(cwb.cpu.pc, before.wrapping_add(1));
    }

    #[test]
    fn test_beq() {
        let mut cwb = get_cpu();
        cwb.cpu.reg.z = true;
        let before = cwb.cpu.pc;
        cwb.beq(1);
        assert_eq!(cwb.cpu.pc, before.wrapping_add(1));
    }

    #[test]
    fn test_bit() {
        let mut cwb = get_cpu();

        cwb.bus.write(0, 0);
        cwb.bit(0);
        assert!(!cwb.cpu.reg.v);
        assert!(!cwb.cpu.reg.n);

        cwb.bus.write(0, 0b1100_0000);
        cwb.bit(0);
        assert!(cwb.cpu.reg.v);
        assert!(cwb.cpu.reg.n);

        cwb.cpu.reg.update_a(0);
        cwb.bus.write(0, 0xFF);
        cwb.bit(0);
        assert!(cwb.cpu.reg.z);

        cwb.cpu.reg.update_a(0x0F);
        cwb.bus.write(0, 0xFF);
        cwb.bit(0);
        assert!(!cwb.cpu.reg.z);
    }

    #[test]
    fn test_bmi() {
        let mut cwb = get_cpu();
        cwb.cpu.reg.n = true;
        let before = cwb.cpu.pc;
        cwb.bmi(1);
        assert_eq!(cwb.cpu.pc, before.wrapping_add(1));
    }

    #[test]
    fn test_bne() {
        let mut cwb = get_cpu();
        cwb.cpu.reg.z = false;
        let before = cwb.cpu.pc;
        cwb.bne(1);
        assert_eq!(cwb.cpu.pc, before.wrapping_add(1));
    }

    #[test]
    fn test_bpl() {
        let mut cwb = get_cpu();
        cwb.cpu.reg.n = false;
        let before = cwb.cpu.pc;
        cwb.bpl(1);
        assert_eq!(cwb.cpu.pc, before.wrapping_add(1));
    }

    #[test]
    fn test_brk() {
        let mut cwb = get_cpu();
        cwb.cpu.pc = 0xABCD;
        cwb.bus.write(u16::from_le_bytes([IRQ_BRK_VECTOR, VECTOR_BASE]), 0xAA);
        cwb.bus.write(u16::from_le_bytes([IRQ_BRK_VECTOR.wrapping_add(1), VECTOR_BASE]), 0xBB);
        cwb.brk();
        assert_eq!(cwb.stack_pop(), 0b0011_0000);
        assert_eq!(cwb.stack_pop(), 0xCE);
        assert_eq!(cwb.stack_pop(), 0xAB);
        println!("{:04x}", cwb.cpu.pc);
        assert_eq!(cwb.cpu.pc, 0xBBAA);
        assert!(cwb.cpu.reg.i);
    }

    #[test]
    fn test_bvc() {
        let mut cwb = get_cpu();
        cwb.cpu.reg.v = false;
        let before = cwb.cpu.pc;
        cwb.bvc(1);
        assert_eq!(cwb.cpu.pc, before.wrapping_add(1));
    }

    #[test]
    fn test_bvs() {
        let mut cwb = get_cpu();
        cwb.cpu.reg.v = true;
        let before = cwb.cpu.pc;
        cwb.bvs(1);
        assert_eq!(cwb.cpu.pc, before.wrapping_add(1));
    }

    #[test]
    fn test_clc() {
        let mut cwb = get_cpu();
        cwb.cpu.reg.c = true;
        cwb.clc();
        assert!(!cwb.cpu.reg.c);
    }

    #[test]
    fn test_cld() {
        let mut cwb = get_cpu();
        cwb.cpu.reg.d = true;
        cwb.cld();
        assert!(!cwb.cpu.reg.d);
    }

    #[test]
    fn test_cli() {
        let mut cwb = get_cpu();
        cwb.cpu.reg.i = true;
        cwb.cli();
        assert!(!cwb.cpu.reg.i);
    }

    #[test]
    fn test_clv() {
        let mut cwb = get_cpu();
        cwb.cpu.reg.v = true;
        cwb.clv();
        assert!(!cwb.cpu.reg.v);
    }

    #[test]
    fn test_cmp_imm() {
        let mut cwb = get_cpu();

        cwb.cpu.reg.update_a(0);
        cwb.cmp_imm(1);
        assert!(!cwb.cpu.reg.c);
        assert!(!cwb.cpu.reg.z);
        assert!(cwb.cpu.reg.n);

        cwb.cpu.reg.update_a(1);
        cwb.cmp_imm(1);
        assert!(cwb.cpu.reg.c);
        assert!(cwb.cpu.reg.z);
        assert!(!cwb.cpu.reg.n);

        cwb.cpu.reg.update_a(2);
        cwb.cmp_imm(1);
        assert!(cwb.cpu.reg.c);
        assert!(!cwb.cpu.reg.z);
        assert!(!cwb.cpu.reg.n);
    }

    #[test]
    fn test_cmp_adr() {
        let mut cwb = get_cpu();
        cwb.bus.write(0xABCD, 1);

        cwb.cpu.reg.update_a(0);
        cwb.cmp_adr(0xABCD);
        assert!(!cwb.cpu.reg.c);
        assert!(!cwb.cpu.reg.z);
        assert!(cwb.cpu.reg.n);

        cwb.cpu.reg.update_a(1);
        cwb.cmp_adr(0xABCD);
        assert!(cwb.cpu.reg.c);
        assert!(cwb.cpu.reg.z);
        assert!(!cwb.cpu.reg.n);

        cwb.cpu.reg.update_a(2);
        cwb.cmp_adr(0xABCD);
        assert!(cwb.cpu.reg.c);
        assert!(!cwb.cpu.reg.z);
        assert!(!cwb.cpu.reg.n);
    }

    #[test]
    fn test_cpx_imm() {
        let mut cwb = get_cpu();

        cwb.cpu.reg.update_x(0);
        cwb.cpx_imm(1);
        assert!(!cwb.cpu.reg.c);
        assert!(!cwb.cpu.reg.z);
        assert!(cwb.cpu.reg.n);

        cwb.cpu.reg.update_x(1);
        cwb.cpx_imm(1);
        assert!(cwb.cpu.reg.c);
        assert!(cwb.cpu.reg.z);
        assert!(!cwb.cpu.reg.n);

        cwb.cpu.reg.update_x(2);
        cwb.cpx_imm(1);
        assert!(cwb.cpu.reg.c);
        assert!(!cwb.cpu.reg.z);
        assert!(!cwb.cpu.reg.n);
    }

    #[test]
    fn test_cpx_adr() {
        let mut cwb = get_cpu();
        cwb.bus.write(0xABCD, 1);

        cwb.cpu.reg.update_x(0);
        cwb.cpx_adr(0xABCD);
        assert!(!cwb.cpu.reg.c);
        assert!(!cwb.cpu.reg.z);
        assert!(cwb.cpu.reg.n);

        cwb.cpu.reg.update_x(1);
        cwb.cpx_adr(0xABCD);
        assert!(cwb.cpu.reg.c);
        assert!(cwb.cpu.reg.z);
        assert!(!cwb.cpu.reg.n);

        cwb.cpu.reg.update_x(2);
        cwb.cpx_adr(0xABCD);
        assert!(cwb.cpu.reg.c);
        assert!(!cwb.cpu.reg.z);
        assert!(!cwb.cpu.reg.n);
    }

    #[test]
    fn test_cpy_imm() {
        let mut cwb = get_cpu();

        cwb.cpu.reg.update_y(0);
        cwb.cpy_imm(1);
        assert!(!cwb.cpu.reg.c);
        assert!(!cwb.cpu.reg.z);
        assert!(cwb.cpu.reg.n);

        cwb.cpu.reg.update_y(1);
        cwb.cpy_imm(1);
        assert!(cwb.cpu.reg.c);
        assert!(cwb.cpu.reg.z);
        assert!(!cwb.cpu.reg.n);

        cwb.cpu.reg.update_y(2);
        cwb.cpy_imm(1);
        assert!(cwb.cpu.reg.c);
        assert!(!cwb.cpu.reg.z);
        assert!(!cwb.cpu.reg.n);
    }

    #[test]
    fn test_cpy_adr() {
        let mut cwb = get_cpu();
        cwb.bus.write(0xABCD, 1);

        cwb.cpu.reg.update_y(0);
        cwb.cpy_adr(0xABCD);
        assert!(!cwb.cpu.reg.c);
        assert!(!cwb.cpu.reg.z);
        assert!(cwb.cpu.reg.n);

        cwb.cpu.reg.update_y(1);
        cwb.cpy_adr(0xABCD);
        assert!(cwb.cpu.reg.c);
        assert!(cwb.cpu.reg.z);
        assert!(!cwb.cpu.reg.n);

        cwb.cpu.reg.update_y(2);
        cwb.cpy_adr(0xABCD);
        assert!(cwb.cpu.reg.c);
        assert!(!cwb.cpu.reg.z);
        assert!(!cwb.cpu.reg.n);
    }

    #[test]
    fn test_dec() {
        let mut cwb = get_cpu();
        let before = cwb.bus.read(0);
        cwb.dec(0);
        assert_eq!(cwb.bus.read(0), before.wrapping_sub(1));
    }

    #[test]
    fn test_dex() {
        let mut cwb = get_cpu();
        let before = cwb.cpu.reg.get_x();
        cwb.dex();
        assert_eq!(cwb.cpu.reg.get_x(), before.wrapping_sub(1));
    }

    #[test]
    fn test_dey() {
        let mut cwb = get_cpu();
        let before = cwb.cpu.reg.get_y();
        cwb.dey();
        assert_eq!(cwb.cpu.reg.get_y(), before.wrapping_sub(1));
    }

    #[test]
    fn test_eor_imm() {
        let mut cwb = get_cpu();

        cwb.cpu.reg.update_a(0x00);
        cwb.eor_imm(0x00);
        assert_eq!(cwb.cpu.reg.get_a(), 0x00);
        assert!(cwb.cpu.reg.z);
        assert!(!cwb.cpu.reg.n);

        cwb.cpu.reg.update_a(0x00);
        cwb.eor_imm(0x01);
        assert_eq!(cwb.cpu.reg.get_a(), 0x01);
        assert!(!cwb.cpu.reg.z);
        assert!(!cwb.cpu.reg.z);

        cwb.cpu.reg.update_a(0x01);
        cwb.eor_imm(0x01);
        assert_eq!(cwb.cpu.reg.get_a(), 0x00);
        assert!(cwb.cpu.reg.z);
        assert!(!cwb.cpu.reg.n);

        cwb.cpu.reg.update_a(0x00);
        cwb.eor_imm(0b1000_0000);
        assert_eq!(cwb.cpu.reg.get_a(), 0b1000_0000);
        assert!(!cwb.cpu.reg.z);
        assert!(cwb.cpu.reg.n);
    }

    #[test]
    fn test_eor_adr() {
        let mut cwb = get_cpu();

        cwb.bus.write(0, 0x00);
        cwb.cpu.reg.update_a(0x00);
        cwb.eor_adr(0);
        assert_eq!(cwb.cpu.reg.get_a(), 0x00);
        assert!(cwb.cpu.reg.z);
        assert!(!cwb.cpu.reg.n);

        cwb.bus.write(0, 0x01);
        cwb.cpu.reg.update_a(0x00);
        cwb.eor_adr(0);
        assert_eq!(cwb.cpu.reg.get_a(), 0x01);
        assert!(!cwb.cpu.reg.z);
        assert!(!cwb.cpu.reg.z);

        cwb.bus.write(0, 0x01);
        cwb.cpu.reg.update_a(0x01);
        cwb.eor_adr(0);
        assert_eq!(cwb.cpu.reg.get_a(), 0x00);
        assert!(cwb.cpu.reg.z);
        assert!(!cwb.cpu.reg.n);

        cwb.bus.write(0, 0b1000_0000);
        cwb.cpu.reg.update_a(0x00);
        cwb.eor_adr(0);
        assert_eq!(cwb.cpu.reg.get_a(), 0b1000_0000);
        assert!(!cwb.cpu.reg.z);
        assert!(cwb.cpu.reg.n);
    }

    #[test]
    fn test_inc() {
        let mut cwb = get_cpu();
        let before = cwb.bus.read(0);
        cwb.inc(0);
        assert_eq!(cwb.bus.read(0), before.wrapping_add(1));
    }

    #[test]
    fn test_inx() {
        let mut cwb = get_cpu();
        let before = cwb.cpu.reg.get_x();
        cwb.inx();
        assert_eq!(cwb.cpu.reg.get_x(), before.wrapping_add(1));
    }

    #[test]
    fn test_iny() {
        let mut cwb = get_cpu();
        let before = cwb.cpu.reg.get_y();
        cwb.iny();
        assert_eq!(cwb.cpu.reg.get_y(), before.wrapping_add(1));
    }

    #[test]
    fn test_jmp() {
        let mut cwb = get_cpu();
        cwb.jmp(0xABCD);
        assert_eq!(cwb.cpu.pc, 0xABCD);
    }

    #[test]
    fn test_jsr() {
        let mut cwb = get_cpu();
        cwb.cpu.pc = 0xABCD;
        cwb.jsr(0x1234);
        assert_eq!(cwb.stack_pop(), 0xCC);
        assert_eq!(cwb.stack_pop(), 0xAB);
        assert_eq!(cwb.cpu.pc, 0x1234);
    }

    #[test]
    fn test_lda_imm() {
        let mut cwb = get_cpu();

        cwb.lda_imm(-1 as i8 as u8);
        assert_eq!(cwb.cpu.reg.get_a(), -1 as i8 as u8);
        assert!(!cwb.cpu.reg.z);
        assert!(cwb.cpu.reg.n);

        cwb.lda_imm(0);
        assert_eq!(cwb.cpu.reg.get_a(), 0);
        assert!(cwb.cpu.reg.z);
        assert!(!cwb.cpu.reg.n);

        cwb.lda_imm(1);
        assert_eq!(cwb.cpu.reg.get_a(), 1);
        assert!(!cwb.cpu.reg.z);
        assert!(!cwb.cpu.reg.n);
    }

    #[test]
    fn test_lda_adr() {
        let mut cwb = get_cpu();

        cwb.bus.write(0, -1 as i8 as u8);
        cwb.lda_adr(0);
        assert_eq!(cwb.cpu.reg.get_a(), -1 as i8 as u8);
        assert!(!cwb.cpu.reg.z);
        assert!(cwb.cpu.reg.n);

        cwb.bus.write(0, 0);
        cwb.lda_adr(0);
        assert_eq!(cwb.cpu.reg.get_a(), 0);
        assert!(cwb.cpu.reg.z);
        assert!(!cwb.cpu.reg.n);

        cwb.bus.write(0, 1);
        cwb.lda_adr(0);
        assert_eq!(cwb.cpu.reg.get_a(), 1);
        assert!(!cwb.cpu.reg.z);
        assert!(!cwb.cpu.reg.n);
    }

    #[test]
    fn test_ldx_imm() {
        let mut cwb = get_cpu();

        cwb.ldx_imm(-1 as i8 as u8);
        assert_eq!(cwb.cpu.reg.get_x(), -1 as i8 as u8);
        assert!(!cwb.cpu.reg.z);
        assert!(cwb.cpu.reg.n);

        cwb.ldx_imm(0);
        assert_eq!(cwb.cpu.reg.get_x(), 0);
        assert!(cwb.cpu.reg.z);
        assert!(!cwb.cpu.reg.n);

        cwb.ldx_imm(1);
        assert_eq!(cwb.cpu.reg.get_x(), 1);
        assert!(!cwb.cpu.reg.z);
        assert!(!cwb.cpu.reg.n);
    }

    #[test]
    fn test_ldx_adr() {
        let mut cwb = get_cpu();

        cwb.bus.write(0, -1 as i8 as u8);
        cwb.ldx_adr(0);
        assert_eq!(cwb.cpu.reg.get_x(), -1 as i8 as u8);
        assert!(!cwb.cpu.reg.z);
        assert!(cwb.cpu.reg.n);

        cwb.bus.write(0, 0);
        cwb.ldx_adr(0);
        assert_eq!(cwb.cpu.reg.get_x(), 0);
        assert!(cwb.cpu.reg.z);
        assert!(!cwb.cpu.reg.n);

        cwb.bus.write(0, 1);
        cwb.ldx_adr(0);
        assert_eq!(cwb.cpu.reg.get_x(), 1);
        assert!(!cwb.cpu.reg.z);
        assert!(!cwb.cpu.reg.n);
    }

    #[test]
    fn test_ldy_imm() {
        let mut cwb = get_cpu();

        cwb.ldy_imm(-1 as i8 as u8);
        assert_eq!(cwb.cpu.reg.get_y(), -1 as i8 as u8);
        assert!(!cwb.cpu.reg.z);
        assert!(cwb.cpu.reg.n);

        cwb.ldy_imm(0);
        assert_eq!(cwb.cpu.reg.get_y(), 0);
        assert!(cwb.cpu.reg.z);
        assert!(!cwb.cpu.reg.n);

        cwb.ldy_imm(1);
        assert_eq!(cwb.cpu.reg.get_y(), 1);
        assert!(!cwb.cpu.reg.z);
        assert!(!cwb.cpu.reg.n);
    }

    #[test]
    fn test_ldy_adr() {
        let mut cwb = get_cpu();

        cwb.bus.write(0, -1 as i8 as u8);
        cwb.ldy_adr(0);
        assert_eq!(cwb.cpu.reg.get_y(), -1 as i8 as u8);
        assert!(!cwb.cpu.reg.z);
        assert!(cwb.cpu.reg.n);

        cwb.bus.write(0, 0);
        cwb.ldy_adr(0);
        assert_eq!(cwb.cpu.reg.get_y(), 0);
        assert!(cwb.cpu.reg.z);
        assert!(!cwb.cpu.reg.n);

        cwb.bus.write(0, 1);
        cwb.ldy_adr(0);
        assert_eq!(cwb.cpu.reg.get_y(), 1);
        assert!(!cwb.cpu.reg.z);
        assert!(!cwb.cpu.reg.n);
    }

    #[test]
    fn test_lsr_imp() {
        let mut cwb = get_cpu();

        cwb.cpu.reg.update_a(0b0000_0010);
        cwb.lsr_imp();
        assert_eq!(cwb.cpu.reg.get_a(), 0b0000_0001);
        assert!(!cwb.cpu.reg.c);
        assert!(!cwb.cpu.reg.z);
        assert!(!cwb.cpu.reg.n);

        cwb.cpu.reg.update_a(0b0000_0001);
        cwb.lsr_imp();
        assert_eq!(cwb.cpu.reg.get_a(), 0b0000_0000);
        assert!(cwb.cpu.reg.c);
        assert!(cwb.cpu.reg.z);
        assert!(!cwb.cpu.reg.n);
    }

    #[test]
    fn test_lsr_adr() {
        let mut cwb = get_cpu();

        cwb.bus.write(0, 0b0000_0010);
        cwb.lsr_adr(0);
        assert_eq!(cwb.bus.read(0), 0b0000_0001);
        assert!(!cwb.cpu.reg.c);
        assert!(!cwb.cpu.reg.z);
        assert!(!cwb.cpu.reg.n);

        cwb.bus.write(0, 0b0000_0001);
        cwb.lsr_adr(0);
        assert_eq!(cwb.cpu.reg.get_a(), 0b0000_0000);
        assert!(cwb.cpu.reg.c);
        assert!(cwb.cpu.reg.z);
        assert!(!cwb.cpu.reg.n);
    }

    #[test]
    fn test_ora_imm() {
        let mut cwb = get_cpu();
        
        cwb.cpu.reg.update_a(0x00);
        cwb.ora_imm(0x00);
        assert_eq!(cwb.cpu.reg.get_a(), 0x00);
        assert!(cwb.cpu.reg.z);
        assert!(!cwb.cpu.reg.n);

        cwb.cpu.reg.update_a(0x00);
        cwb.ora_imm(0x01);
        assert_eq!(cwb.cpu.reg.get_a(), 0x01);
        assert!(!cwb.cpu.reg.z);
        assert!(!cwb.cpu.reg.n);

        cwb.cpu.reg.update_a(0x00);
        cwb.ora_imm(0b1000_0000);
        assert_eq!(cwb.cpu.reg.get_a(), 0b1000_0000);
        assert!(!cwb.cpu.reg.z);
        assert!(cwb.cpu.reg.n);
    }

    #[test]
    fn test_ora_adr() {
        let mut cwb = get_cpu();
        
        cwb.cpu.reg.update_a(0x00);
        cwb.bus.write(0, 0x00);
        cwb.ora_adr(0);
        assert_eq!(cwb.cpu.reg.get_a(), 0x00);
        assert!(cwb.cpu.reg.z);
        assert!(!cwb.cpu.reg.n);

        cwb.cpu.reg.update_a(0x00);
        cwb.bus.write(0, 0x01);
        cwb.ora_adr(0);
        assert_eq!(cwb.cpu.reg.get_a(), 0x01);
        assert!(!cwb.cpu.reg.z);
        assert!(!cwb.cpu.reg.n);

        cwb.cpu.reg.update_a(0x00);
        cwb.bus.write(0, 0b1000_0000);
        cwb.ora_adr(0);
        assert_eq!(cwb.cpu.reg.get_a(), 0b1000_0000);
        assert!(!cwb.cpu.reg.z);
        assert!(cwb.cpu.reg.n);
    }

    #[test]
    fn test_pha() {
        let mut cwb = get_cpu();
        cwb.cpu.reg.update_a(1);
        cwb.pha();
        assert_eq!(cwb.stack_pop(), 1);
    }

    #[test]
    fn test_php() {
        let mut cwb = get_cpu();
        cwb.cpu.reg.c = true;
        cwb.php();
        assert_eq!(cwb.stack_pop(), cwb.cpu.reg.get_status(false));
    }

    #[test]
    fn test_pla() {
        let mut cwb = get_cpu();
        cwb.stack_push(1);
        cwb.pla();
        assert_eq!(cwb.cpu.reg.get_a(), 1);
    }

    #[test]
    fn test_plp() {
        let mut cwb = get_cpu();
        cwb.stack_push(1);
        cwb.plp();
        assert!(cwb.cpu.reg.c);
    }

    #[test]
    fn test_rol_imp() {
        let mut cwb = get_cpu();

        cwb.cpu.reg.update_a(0b0000_0001);
        cwb.cpu.reg.c = false;
        cwb.rol_imp();
        assert_eq!(cwb.cpu.reg.get_a(), 0b0000_0010);
        assert!(!cwb.cpu.reg.c);
        assert!(!cwb.cpu.reg.z);
        assert!(!cwb.cpu.reg.n);

        cwb.cpu.reg.update_a(0b1000_0000);
        cwb.cpu.reg.c = false;
        cwb.rol_imp();
        assert_eq!(cwb.cpu.reg.get_a(), 0b0000_0000);
        assert!(cwb.cpu.reg.c);
        assert!(cwb.cpu.reg.z);
        assert!(!cwb.cpu.reg.n);

        cwb.cpu.reg.update_a(0b0100_0000);
        cwb.cpu.reg.c = false;
        cwb.rol_imp();
        assert_eq!(cwb.cpu.reg.get_a(), 0b1000_0000);
        assert!(!cwb.cpu.reg.c);
        assert!(!cwb.cpu.reg.z);
        assert!(cwb.cpu.reg.n);

        cwb.cpu.reg.update_a(0b0000_0000);
        cwb.cpu.reg.c = true;
        cwb.rol_imp();
        assert_eq!(cwb.cpu.reg.get_a(), 0b0000_0001);
        assert!(!cwb.cpu.reg.c);
        assert!(!cwb.cpu.reg.z);
        assert!(!cwb.cpu.reg.n);
    }

    #[test]
    fn test_rol_adr() {
        let mut cwb = get_cpu();

        cwb.bus.write(0, 0b0000_0001);
        cwb.cpu.reg.c = false;
        cwb.rol_adr(0);
        assert_eq!(cwb.bus.read(0), 0b0000_0010);
        assert!(!cwb.cpu.reg.c);
        assert!(!cwb.cpu.reg.z);
        assert!(!cwb.cpu.reg.n);

        cwb.bus.write(0, 0b1000_0000);
        cwb.cpu.reg.c = false;
        cwb.rol_adr(0);
        assert_eq!(cwb.bus.read(0), 0b0000_0000);
        assert!(cwb.cpu.reg.c);
        assert!(cwb.cpu.reg.z);
        assert!(!cwb.cpu.reg.n);

        cwb.bus.write(0, 0b0100_0000);
        cwb.cpu.reg.c = false;
        cwb.rol_adr(0);
        assert_eq!(cwb.bus.read(0), 0b1000_0000);
        assert!(!cwb.cpu.reg.c);
        assert!(!cwb.cpu.reg.z);
        assert!(cwb.cpu.reg.n);

        cwb.bus.write(0, 0b0000_0000);
        cwb.cpu.reg.c = true;
        cwb.rol_adr(0);
        assert_eq!(cwb.bus.read(0), 0b0000_0001);
        assert!(!cwb.cpu.reg.c);
        assert!(!cwb.cpu.reg.z);
        assert!(!cwb.cpu.reg.n);
    }

    #[test]
    fn test_ror_imp() {
        let mut cwb = get_cpu();

        cwb.cpu.reg.update_a(0b0000_0010);
        cwb.cpu.reg.c = false;
        cwb.ror_imp();
        assert_eq!(cwb.cpu.reg.get_a(), 0b0000_0001);
        assert!(!cwb.cpu.reg.c);
        assert!(!cwb.cpu.reg.z);
        assert!(!cwb.cpu.reg.n);

        cwb.cpu.reg.update_a(0b0000_0001);
        cwb.cpu.reg.c = false;
        cwb.ror_imp();
        assert_eq!(cwb.cpu.reg.get_a(), 0b0000_0000);
        assert!(cwb.cpu.reg.c);
        assert!(cwb.cpu.reg.z);
        assert!(!cwb.cpu.reg.n);

        cwb.cpu.reg.update_a(0b0000_0000);
        cwb.cpu.reg.c = true;
        cwb.ror_imp();
        assert_eq!(cwb.cpu.reg.get_a(), 0b1000_0000);
        assert!(!cwb.cpu.reg.c);
        assert!(!cwb.cpu.reg.z);
        assert!(cwb.cpu.reg.n);
    }

    #[test]
    fn test_ror_adr() {
        let mut cwb = get_cpu();

        cwb.bus.write(0, 0b0000_0010);
        cwb.cpu.reg.c = false;
        cwb.ror_adr(0);
        assert_eq!(cwb.bus.read(0), 0b0000_0001);
        assert!(!cwb.cpu.reg.c);
        assert!(!cwb.cpu.reg.z);
        assert!(!cwb.cpu.reg.n);

        cwb.bus.write(0, 0b0000_0001);
        cwb.cpu.reg.c = false;
        cwb.ror_adr(0);
        assert_eq!(cwb.bus.read(0), 0b0000_0000);
        assert!(cwb.cpu.reg.c);
        assert!(cwb.cpu.reg.z);
        assert!(!cwb.cpu.reg.n);

        cwb.bus.write(0, 0b0000_0000);
        cwb.cpu.reg.c = true;
        cwb.ror_adr(0);
        assert_eq!(cwb.bus.read(0), 0b1000_0000);
        assert!(!cwb.cpu.reg.c);
        assert!(!cwb.cpu.reg.z);
        assert!(cwb.cpu.reg.n);
    }

    #[test]
    fn test_rti() {
        let mut cwb = get_cpu();
        cwb.stack_push(0xFF);
        cwb.stack_push(0x0A);
        cwb.stack_push(0b0000_0001);
        cwb.rti();
        assert!(cwb.cpu.reg.c);
        assert_eq!(cwb.cpu.pc, 0xFF0A);
    }

    #[test]
    fn test_rts() {
        let mut cwb = get_cpu();
        cwb.stack_push(0xFF);
        cwb.stack_push(0x0A);
        cwb.rts();
        assert_eq!(cwb.cpu.pc, 0xFF0B);
    }

    #[test]
    fn test_sec() {
        let mut cwb = get_cpu();
        cwb.sec();
        assert!(cwb.cpu.reg.c);
    }

    #[test]
    fn test_sed() {
        let mut cwb = get_cpu();
        cwb.sed();
        assert!(cwb.cpu.reg.d);
    }

    #[test]
    fn test_sei() {
        let mut cwb = get_cpu();
        cwb.sei();
        assert!(cwb.cpu.reg.i);
    }

    #[test]
    fn test_sta() {
        let mut cwb = get_cpu();
        cwb.cpu.reg.update_a(1);
        cwb.sta(0xABCD);
        assert_eq!(cwb.bus.read(0xABCD), 1);
    }

    #[test]
    fn test_stx() {
        let mut cwb = get_cpu();
        cwb.cpu.reg.update_x(1);
        cwb.stx(0xABCD);
        assert_eq!(cwb.bus.read(0xABCD), 1);
    }

    #[test]
    fn test_sty() {
        let mut cwb = get_cpu();
        cwb.cpu.reg.update_y(1);
        cwb.sty(0xABCD);
        assert_eq!(cwb.bus.read(0xABCD), 1);
    }

    #[test]
    fn test_tax() {
        let mut cwb = get_cpu();
        cwb.cpu.reg.update_a(1);
        cwb.tax();
        assert_eq!(cwb.cpu.reg.get_x(), 1);
    }

    #[test]
    fn test_tay() {
        let mut cwb = get_cpu();
        cwb.cpu.reg.update_a(1);
        cwb.tay();
        assert_eq!(cwb.cpu.reg.get_y(), 1);
    }

    #[test]
    fn test_tsx() {
        let mut cwb = get_cpu();
        cwb.cpu.sp = 1;
        cwb.tsx();
        assert_eq!(cwb.cpu.reg.get_x(), 1);
    }

    #[test]
    fn test_txa() {
        let mut cwb = get_cpu();
        cwb.cpu.reg.update_x(1);
        cwb.txa();
        assert_eq!(cwb.cpu.reg.get_a(), 1);
    }

    #[test]
    fn test_txs() {
        let mut cwb = get_cpu();
        cwb.cpu.reg.update_x(1);
        cwb.txs();
        assert_eq!(cwb.cpu.sp, 1);
    }

    #[test]
    fn test_tya() {
        let mut cwb = get_cpu();
        cwb.cpu.reg.update_y(1);
        cwb.tya();
        assert_eq!(cwb.cpu.reg.get_a(), 1);
    }
}