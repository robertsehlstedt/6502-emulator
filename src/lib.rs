pub mod instruction;

mod cpu;
mod registers;

pub trait Variant {
    fn decode(opcode: u8) -> Option<(
        crate::instruction::InstructionCode,
        crate::instruction::AddressingMode
    )>;
}

pub trait Bus {
    fn read(&self, addr: u16) -> u8;
    fn write(&mut self, addr: u16, value: u8);
}

#[derive(Default, Clone, Copy)]
pub struct Cpu<V> {
    pub reg: registers::RegisterState,
    pub pc: u16,
    pub sp: u8,

    _variant: core::marker::PhantomData<V>,
}

impl<V: Variant> Cpu<V> {
    pub fn new(_: V) -> Self {
        Cpu {
            reg: registers::RegisterState::default(),
            pc: 0,
            sp: 0,
            _variant: core::marker::PhantomData::<V>,
        }
    }

    pub fn step(&mut self, bus: &mut impl Bus) {
        cpu::CpuWithBus {cpu: self, bus}.step()
    }

    pub fn reset(&mut self, bus: &mut impl Bus) {
        cpu::CpuWithBus {cpu: self, bus}.reset()
    }

    pub fn irq(&mut self, bus: &mut impl Bus) {
        cpu::CpuWithBus {cpu: self, bus}.irq()
    }

    pub fn nmi(&mut self, bus: &mut impl Bus) {
        cpu::CpuWithBus {cpu: self, bus}.nmi()
    }
}