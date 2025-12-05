const MEM_SIZE: usize = 8192;

pub struct Memory {
    mem: [u8; MEM_SIZE],
}

impl Default for Memory {
    fn default() -> Self {
        Memory { mem: [0; MEM_SIZE] }
    }
}

// todo bound checks
impl Memory {
    pub fn load_byte(&self, addr: u32) -> u8 {
        self.mem[addr as usize]
    }

    pub fn load_half(&self, addr: u32) -> u16 {
        u16::from_le_bytes([self.load_byte(addr), self.load_byte(addr + 1)])
    }

    pub fn load_word(&self, addr: u32) -> u32 {
        u32::from_le_bytes(
            (&self.mem[addr as usize..addr as usize + 4])
                .try_into()
                .unwrap(),
        )
    }

    pub fn store_byte(&mut self, addr: u32, value: u8) {
        self.mem[addr as usize] = value;
    }

    pub fn store_half(&mut self, addr: u32, value: u16) {
        self.mem[addr as usize..addr as usize + 2].copy_from_slice(value.to_le_bytes().as_slice());
    }

    pub fn store_word(&mut self, addr: u32, value: u32) {
        self.mem[addr as usize..addr as usize + 4].copy_from_slice(value.to_le_bytes().as_slice());
    }
}
