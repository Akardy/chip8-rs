pub const SIZE: usize = 4096;
pub struct Memory {
    memory: [u8; SIZE]
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            memory: [0; SIZE]
        }
    }

    pub fn read(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }
}