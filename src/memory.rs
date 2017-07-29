#[derive(Debug, Clone)]
pub struct Memory {
    data: Vec<u8>,
}

impl Memory {
    pub fn new(mem_size: usize) -> Memory {
        Memory {
            data: vec![0; mem_size]
        }
    }

    pub fn from_data(data: Vec<u8>) -> Memory {
        Memory {
            data
        }
    }

    #[inline]
    pub fn get_byte(&self, index: u32) -> u8 {
        let index = index as usize;

        self.data[index]
    }

    #[inline]
    pub fn set_byte(&mut self, index: u32, byte: u8) {
        let index = index as usize;

        self.data[index] = byte;
    }

    pub fn get_half_word(&self, index: u32) -> u16 {
        let _0 = self.get_byte(index + 0) as u16;
        let _1 = self.get_byte(index + 1) as u16;

        _0 | (_1 << 8)
    }

    pub fn set_half_word(&mut self, index: u32, half_word: u16) {
        self.set_byte(index + 0, half_word as u8);
        self.set_byte(index + 1, (half_word >> 8) as u8);
    }

    pub fn get_word(&self, index: u32) -> u32 {
        let _0 = self.get_byte(index + 0) as u32;
        let _1 = self.get_byte(index + 1) as u32;
        let _2 = self.get_byte(index + 2) as u32;
        let _3 = self.get_byte(index + 3) as u32;

        _0 | (_1 << 8) | (_2 << 16) | (_3 << 24)
    } 

    pub fn set_word(&mut self, index: u32, word: u32) {
        self.set_byte(index + 0, word as u8);
        self.set_byte(index + 1, (word >> 8) as u8);
        self.set_byte(index + 2, (word >> 16) as u8);
        self.set_byte(index + 3, (word >> 24) as u8);
    }
}
