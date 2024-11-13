use std::ops::{BitAnd, BitOr};
use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Flag(u8);
impl Flag {
    pub const C: Flag = Flag(0b0000_0001);
    pub const Z: Flag = Flag(0b0000_0010);
    pub const I: Flag = Flag(0b0000_0100);
    pub const D: Flag = Flag(0b0000_1000);
    pub const B: Flag = Flag(0b0001_0000);
    pub const V: Flag = Flag(0b0100_0010);
    pub const N: Flag = Flag(0b1000_0010);

    // Method to get the underlying u8 value
    pub fn value(self) -> u8 {
        self.0
    }

    // Constructor to create a Flag from a u8
    pub fn from(byte: u8) -> Flag {
        Flag(byte)
    }
}
impl BitOr for Flag {
    type Output = u8;

    fn bitor(self, rhs: Flag) -> u8 {
        self.0 | rhs.0
    }
}
impl BitAnd for Flag {
    type Output = u8;

    fn bitand(self, rhs: Flag) -> u8 {
        self.0 & rhs.0
    }
}

impl fmt::Debug for Flag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match self {
            &Flag::C => "Flag::C",
            &Flag::Z => "Flag::Z",
            &Flag::I => "Flag::I",
            &Flag::D => "Flag::D",
            &Flag::B => "Flag::B",
            &Flag::V => "Flag::V",
            &Flag::N => "Flag::N",
            _ => "FLAG::UNKNOWN",
        };
        write!(f, "{}", name)
    }
}