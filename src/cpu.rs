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

            (InstructionCode::BCC, OperationInput::REL(offset)) => todo!(),
            
            (InstructionCode::BCS, OperationInput::REL(offset)) => todo!(),

            (InstructionCode::BEQ, OperationInput::REL(offset)) => todo!(),

            (InstructionCode::BIT, OperationInput::REL(offset)) => todo!(),

            (InstructionCode::BMI, OperationInput::REL(offset)) => todo!(),

            (InstructionCode::BNE, OperationInput::REL(offset)) => todo!(),

            (InstructionCode::BPL, OperationInput::REL(offset)) => todo!(),

            (InstructionCode::BRK, OperationInput::REL(offset)) => todo!(),

            (InstructionCode::BVC, OperationInput::REL(offset)) => todo!(),

            (InstructionCode::BVS, OperationInput::REL(offset)) => todo!(),

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

            (InstructionCode::DEC, OperationInput::ADR(addr)) => todo!(),

            (InstructionCode::DEX, OperationInput::IMP) => todo!(),

            (InstructionCode::DEY, OperationInput::IMP) => todo!(),

            (InstructionCode::EOR, OperationInput::IMM(val)) => todo!(),
            (InstructionCode::EOR, OperationInput::ADR(addr)) => todo!(),

            (InstructionCode::INC, OperationInput::IMM(val)) => todo!(),
            (InstructionCode::INC, OperationInput::ADR(addr)) => todo!(),

            (InstructionCode::INX, OperationInput::IMP) => todo!(),

            (InstructionCode::INY, OperationInput::IMP) => todo!(),

            (InstructionCode::JMP, OperationInput::IMM(val)) => todo!(),
            (InstructionCode::JMP, OperationInput::ADR(addr)) => todo!(),

            (InstructionCode::JSR, OperationInput::ADR(addr)) => todo!(),

            (InstructionCode::LDA, OperationInput::IMM(val)) => todo!(),
            (InstructionCode::LDA, OperationInput::ADR(addr)) => todo!(),

            (InstructionCode::LDX, OperationInput::IMM(val)) => todo!(),
            (InstructionCode::LDX, OperationInput::ADR(addr)) => todo!(),

            (InstructionCode::LDY, OperationInput::IMM(val)) => todo!(),
            (InstructionCode::LDY, OperationInput::ADR(addr)) => todo!(),

            (InstructionCode::LSR, OperationInput::IMP) => todo!(),
            (InstructionCode::LSR, OperationInput::ADR(addr)) => todo!(),

            (InstructionCode::NOP, OperationInput::IMP) => todo!(),

            (InstructionCode::ORA, OperationInput::IMM(val)) => todo!(),
            (InstructionCode::ORA, OperationInput::ADR(addr)) => todo!(),

            (InstructionCode::PHA, OperationInput::IMP) => todo!(),

            (InstructionCode::PHP, OperationInput::IMP) => todo!(),

            (InstructionCode::PLA, OperationInput::IMP) => todo!(),

            (InstructionCode::PLP, OperationInput::IMP) => todo!(),

            (InstructionCode::ROL, OperationInput::IMP) => todo!(),
            (InstructionCode::ROL, OperationInput::ADR(addr)) => todo!(),

            (InstructionCode::ROR, OperationInput::IMP) => todo!(),
            (InstructionCode::ROR, OperationInput::ADR(addr)) => todo!(),

            (InstructionCode::RTI, OperationInput::IMP) => todo!(),

            (InstructionCode::RTS, OperationInput::IMP) => todo!(),

            (InstructionCode::SBC, OperationInput::IMM(val)) => todo!(),
            (InstructionCode::SBC, OperationInput::ADR(addr)) => todo!(),

            (InstructionCode::SEC, OperationInput::IMP) => todo!(),

            (InstructionCode::SED, OperationInput::IMP) => todo!(),

            (InstructionCode::SEI, OperationInput::IMP) => todo!(),

            (InstructionCode::STA, OperationInput::ADR(addr)) => todo!(),

            (InstructionCode::STX, OperationInput::ADR(addr)) => todo!(),

            (InstructionCode::STY, OperationInput::ADR(addr)) => todo!(),

            (InstructionCode::TAX, OperationInput::IMP) => todo!(),

            (InstructionCode::TAY, OperationInput::IMP) => todo!(),

            (InstructionCode::TSX, OperationInput::IMP) => todo!(),

            (InstructionCode::TXA, OperationInput::IMP) => todo!(),

            (InstructionCode::TXS, OperationInput::IMP) => todo!(),

            (InstructionCode::TYA, OperationInput::IMP) => todo!(),

            _illegal => panic!(),
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

}