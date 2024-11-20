use crate::registers::RegisterState;
use crate::Variant;
use crate::instruction::{AddressingMode, OperationInput, Instruction};

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

    fn execute_operation(&mut self, instruction: Instruction) {

    }
}