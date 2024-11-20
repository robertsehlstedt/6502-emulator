pub mod cpu;
pub mod instruction;
pub mod registers;

pub trait Variant {
    fn decode(opcode: u8) -> Option<(
        crate::instruction::InstructionCode,
        crate::instruction::AddressingMode
    )>;
}