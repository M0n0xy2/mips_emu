use std::collections::HashMap;
use std::fmt;
use std::clone::Clone;

#[derive(Debug, Clone)]
pub struct Memory {
    blocks: HashMap<usize, Block>
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            blocks: HashMap::new()
        }
    }

    #[inline]
    pub fn get_byte(&self, index: u32) -> u8 {
        let (block_id, data_id) = get_ids(index);

        if let Some(block) = self.blocks.get(&block_id) {
            block.data[data_id]
        } else {
            0
        }
    }

    #[inline]
    pub fn set_byte(&mut self, index: u32, byte: u8) {
        let (block_id, data_id) = get_ids(index);

        self.blocks
            .entry(block_id)
            .or_insert_with(Block::new).data[data_id] = byte;
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

const BLOCK_BIT_LEN: usize = 8;

struct Block {
    pub data: [u8; 1 << BLOCK_BIT_LEN]
}

impl Block {
    pub fn new() -> Self {
        Block {
            data: [0; 1 << BLOCK_BIT_LEN]
        }
    }
}

impl fmt::Debug for Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Block {{ data: {:?} }}", &&self.data[..])
    }
}

impl Clone for Block {
    fn clone(&self) -> Block {
        let mut data = [0; 1 << BLOCK_BIT_LEN];
        for i in 0..(1 << BLOCK_BIT_LEN) {
            data[i] = self.data[i];
        }

        Block {
            data
        }
    }
}

fn get_ids(index: u32) -> (usize, usize) {
    let block_id = (index >> BLOCK_BIT_LEN) as usize;
    let data_id = (index & !(0xFFFFFFFF << BLOCK_BIT_LEN)) as usize;

    (block_id, data_id)
}
