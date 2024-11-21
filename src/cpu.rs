use crate::registers::RegisterState;
use crate::Variant;
use crate::instruction::{AddressingMode, OperationInput, InstructionCode, Instruction};

const STACK_BASE:       u8 = 0x01;
const VECTOR_BASE:      u8 = 0xFF;
const IRQ_BRK_VECTOR:   u8 = 0xFE;
const RESET_VECTOR:     u8 = 0xFC;
const NMI_VECTOR:       u8 = 0xFA;

pub trait Bus {
    fn read(&mut self, addr: u16) -> u8;
    fn write(&mut self, addr: u16, value: u8);
}

#[derive(Default, Clone, Copy)]
pub struct Cpu<V> {
    pub reg: RegisterState,
    pub pc: u16,
    pub sp: u8,

    _variant: core::marker::PhantomData<V>,
}

impl<V: Variant> Cpu<V> {
    pub fn new(_: V) -> Self {
        Cpu {
            reg: RegisterState::default(),
            pc: 0,
            sp: 0,
            _variant: core::marker::PhantomData::<V>,
        }
    }

    pub fn step(&mut self, bus: &mut impl Bus) {
        CpuWithBus {cpu: self, bus}.step()
    }

    pub fn reset(&mut self, bus: &mut impl Bus) {
        CpuWithBus {cpu: self, bus}.reset()
    }

    pub fn irq(&mut self, bus: &mut impl Bus) {
        CpuWithBus {cpu: self, bus}.irq()
    }

    pub fn nmi(&mut self, bus: &mut impl Bus) {
        CpuWithBus {cpu: self, bus}.nmi()
    }
}

struct CpuWithBus<'c, B, V> {
    cpu: &'c mut Cpu<V>,
    bus: &'c mut B,
}

impl<B: Bus, V: Variant> CpuWithBus<'_, B, V> {
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

    fn reset(&mut self) {
        self.cpu.reg.i = true;
        self.cpu.sp = self.cpu.sp.wrapping_add(3);
        self.cpu.pc = self.read_u16(VECTOR_BASE, RESET_VECTOR);
    }
    
    fn irq(&mut self) {}
    fn nmi(&mut self) {}

    fn step(&mut self) {
        let (instr_code, addr_mode) = V::decode(self.take_u8_at_pc()).unwrap();
        let op_input = self.execute_addressing(addr_mode);
        self.execute_operation((instr_code, op_input));
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
            (InstructionCode::ADC, OperationInput::IMM(val)) => todo!(),
            (InstructionCode::ADC, OperationInput::ADR(addr)) => todo!(),

            (InstructionCode::AND, OperationInput::IMM(val)) => todo!(),
            (InstructionCode::AND, OperationInput::ADR(addr)) => todo!(),

            (InstructionCode::ASL, OperationInput::IMP) => todo!(),
            (InstructionCode::ASL, OperationInput::ADR(val)) => todo!(),

            (InstructionCode::BCC, OperationInput::REL(offset)) => self.bcc(offset),
            
            (InstructionCode::BCS, OperationInput::REL(offset)) => self.bcs(offset),

            (InstructionCode::BEQ, OperationInput::REL(offset)) => self.beq(offset),

            (InstructionCode::BIT, OperationInput::REL(offset)) => todo!(),

            (InstructionCode::BMI, OperationInput::REL(offset)) => self.bmi(offset),

            (InstructionCode::BNE, OperationInput::REL(offset)) => self.bne(offset),

            (InstructionCode::BPL, OperationInput::REL(offset)) => self.bpl(offset),

            (InstructionCode::BRK, OperationInput::REL(offset)) => todo!(),

            (InstructionCode::BVC, OperationInput::REL(offset)) => self.bvc(offset),

            (InstructionCode::BVS, OperationInput::REL(offset)) => self.bvs(offset),

            (InstructionCode::CLC, OperationInput::IMP) => self.clc(),

            (InstructionCode::CLD, OperationInput::IMP) => self.cld(),

            (InstructionCode::CLI, OperationInput::IMP) => self.cli(),

            (InstructionCode::CLV, OperationInput::IMP) => self.clv(),

            (InstructionCode::CMP, OperationInput::IMM(val)) => todo!(),
            (InstructionCode::CMP, OperationInput::ADR(addr)) => todo!(),

            (InstructionCode::CPX, OperationInput::IMM(val)) => todo!(),
            (InstructionCode::CPX, OperationInput::ADR(addr)) => todo!(),

            (InstructionCode::CPY, OperationInput::IMM(val)) => todo!(),
            (InstructionCode::CPY, OperationInput::ADR(addr)) => todo!(),

            (InstructionCode::DEC, OperationInput::ADR(addr)) => self.dec(addr),

            (InstructionCode::DEX, OperationInput::IMP) => self.dex(),

            (InstructionCode::DEY, OperationInput::IMP) => self.dey(),

            (InstructionCode::EOR, OperationInput::IMM(val)) => todo!(),
            (InstructionCode::EOR, OperationInput::ADR(addr)) => todo!(),

            (InstructionCode::INC, OperationInput::ADR(addr)) => self.inc(addr),

            (InstructionCode::INX, OperationInput::IMP) => self.inx(),

            (InstructionCode::INY, OperationInput::IMP) => self.iny(),

            (InstructionCode::JMP, OperationInput::IMM(val)) => todo!(),
            (InstructionCode::JMP, OperationInput::ADR(addr)) => todo!(),

            (InstructionCode::JSR, OperationInput::ADR(addr)) => self.jsr(addr),

            (InstructionCode::LDA, OperationInput::IMM(val)) => todo!(),
            (InstructionCode::LDA, OperationInput::ADR(addr)) => todo!(),

            (InstructionCode::LDX, OperationInput::IMM(val)) => todo!(),
            (InstructionCode::LDX, OperationInput::ADR(addr)) => todo!(),

            (InstructionCode::LDY, OperationInput::IMM(val)) => todo!(),
            (InstructionCode::LDY, OperationInput::ADR(addr)) => todo!(),

            (InstructionCode::LSR, OperationInput::IMP) => todo!(),
            (InstructionCode::LSR, OperationInput::ADR(addr)) => todo!(),

            (InstructionCode::NOP, OperationInput::IMP) => self.nop(),

            (InstructionCode::ORA, OperationInput::IMM(val)) => todo!(),
            (InstructionCode::ORA, OperationInput::ADR(addr)) => todo!(),

            (InstructionCode::PHA, OperationInput::IMP) => self.pha(),

            (InstructionCode::PHP, OperationInput::IMP) => self.php(),

            (InstructionCode::PLA, OperationInput::IMP) => self.pla(),

            (InstructionCode::PLP, OperationInput::IMP) => self.plp(),

            (InstructionCode::ROL, OperationInput::IMP) => todo!(),
            (InstructionCode::ROL, OperationInput::ADR(addr)) => todo!(),

            (InstructionCode::ROR, OperationInput::IMP) => todo!(),
            (InstructionCode::ROR, OperationInput::ADR(addr)) => todo!(),

            (InstructionCode::RTI, OperationInput::IMP) => self.rti(),

            (InstructionCode::RTS, OperationInput::IMP) => self.rts(),

            (InstructionCode::SBC, OperationInput::IMM(val)) => todo!(),
            (InstructionCode::SBC, OperationInput::ADR(addr)) => todo!(),

            (InstructionCode::SEC, OperationInput::IMP) => self.sec(),

            (InstructionCode::SED, OperationInput::IMP) => self.sed(),

            (InstructionCode::SEI, OperationInput::IMP) => self.sei(),

            (InstructionCode::STA, OperationInput::ADR(addr)) => todo!(),

            (InstructionCode::STX, OperationInput::ADR(addr)) => todo!(),

            (InstructionCode::STY, OperationInput::ADR(addr)) => todo!(),

            (InstructionCode::TAX, OperationInput::IMP) => self.tax(),

            (InstructionCode::TAY, OperationInput::IMP) => self.tay(),

            (InstructionCode::TSX, OperationInput::IMP) => self.tsx(),

            (InstructionCode::TXA, OperationInput::IMP) => self.txa(),

            (InstructionCode::TXS, OperationInput::IMP) => self.txs(),

            (InstructionCode::TYA, OperationInput::IMP) => self.tya(),

            _illegal => panic!(),
        }
    }

    fn bcc(&mut self, offset: u16) {
        if (!self.cpu.reg.c) {
            let addr = self.cpu.pc.wrapping_add(offset);
            self.cpu.pc = addr;
        }
    }

    fn bcs(&mut self, offset: u16) {
        if (self.cpu.reg.c) {
            let addr = self.cpu.pc.wrapping_add(offset);
            self.cpu.pc = addr;
        }
    }

    fn beq(&mut self, offset: u16) {
        if (self.cpu.reg.z) {
            let addr = self.cpu.pc.wrapping_add(offset);
            self.cpu.pc = addr;
        }
    }

    fn bmi(&mut self, offset: u16) {
        if (self.cpu.reg.n) {
            let addr = self.cpu.pc.wrapping_add(offset);
            self.cpu.pc = addr;
        }
    }

    fn bne(&mut self, offset: u16) {
        if (!self.cpu.reg.z) {
            let addr = self.cpu.pc.wrapping_add(offset);
            self.cpu.pc = addr;
        }
    }

    fn bpl(&mut self, offset: u16) {
        if (!self.cpu.reg.n) {
            let addr = self.cpu.pc.wrapping_add(offset);
            self.cpu.pc = addr;
        }
    }

    fn bvc(&mut self, offset: u16) {
        if (!self.cpu.reg.v) {
            let addr = self.cpu.pc.wrapping_add(offset);
            self.cpu.pc = addr;
        }
    }

    fn bvs(&mut self, offset: u16) {
        if (self.cpu.reg.v) {
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

    fn jsr(&mut self, addr: u16) {
        let return_addr = self.cpu.pc.wrapping_sub(1);
        let [ret_low, ret_high] = return_addr.to_le_bytes();
        self.stack_push(ret_high);
        self.stack_push(ret_low);
        self.cpu.pc = addr;
    }

    fn nop(&self) { }

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

    struct MockBus {
        memory: [u8; u16::MAX as usize],
    }
    impl Bus for MockBus {
        fn read(&mut self, addr: u16) -> u8 { self.memory[addr as usize] }
        fn write(&mut self, addr: u16, value: u8) { self.memory[addr as usize] = value }
    }

    struct MockVariant;
    impl Variant for MockVariant {
        fn decode(opcode: u8) -> Option<(
            crate::instruction::InstructionCode,
            crate::instruction::AddressingMode
        )> {
            None
        }
    }

    fn get_cpu() -> CpuWithBus<'static, MockBus, MockVariant> {
        let cpu = Box::leak(Box::new(Cpu::new(MockVariant)));
        let bus = Box::leak(Box::new(MockBus { memory: [0; u16::MAX as usize] }));
        CpuWithBus {cpu: cpu, bus: bus}
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
    fn test_jsr() {
        let mut cwb = get_cpu();
        cwb.cpu.pc = 0xABCD;
        cwb.jsr(0x1234);
        assert_eq!(cwb.stack_pop(), 0xCC);
        assert_eq!(cwb.stack_pop(), 0xAB);
        assert_eq!(cwb.cpu.pc, 0x1234);
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