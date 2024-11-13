use std::ops::{Index, IndexMut};

#[derive(Debug)]
pub struct Memory {
    data: [u8; Memory::MAX_MEM],
}

impl Memory {
    pub const MAX_MEM: usize = 1024 * 64;
    
    pub fn new() -> Self {
        Self {
            data: [0; Memory::MAX_MEM]
        }
    }
}

impl Index<usize> for Memory {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl IndexMut<usize> for Memory {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}