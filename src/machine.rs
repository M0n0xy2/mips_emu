use std::path::Path;
use elf;

pub const MEM_SIZE: usize = 1 << 24;

#[derive(Debug, Clone)]
pub struct Machine {
    registers: [u32; 31],
    pub hi: u32,
    pub lo: u32,
    pub data: Vec<u8>,
}

impl Machine {
    pub fn new(data: Vec<u8>) -> Machine {
        Machine {
            registers: [0; 31],
            hi: 0,
            lo: 0,
            data,
        }
    }

    pub fn get_register(&self, index: u32) -> u32 {
        assert!(index < 32, "Index out of bounds");

        match index as usize {
            0 => 0,
            val => self.registers[val - 1],
        }
    }

    pub fn set_register(&mut self, index: u32, value: u32) {
        assert!(index < 32, "Index out of bounds");
        let index = index as usize;

        match index as usize {
            0 => {},
            val => self.registers[val - 1] = value,
        }
    }

    pub fn get_byte(&self, index: u32) -> u8 {
        let index = index as usize;

        self.data[index]
    }

    pub fn set_byte(&mut self, index: u32, byte: u8) {
        let index = index as usize;

        self.data[index] = byte;
    }

    pub fn get_half_word(&self, index: u32) -> u16 {
        let index = index as usize;

        let _0 = self.data[index + 0] as u16;
        let _1 = self.data[index + 1] as u16;

        _0 | (_1 << 8)
    }

    pub fn set_half_word(&mut self, index: u32, half_word: u16) {
        let index = index as usize;

        self.data[index + 0] = half_word as u8;
        self.data[index + 1] = (half_word >> 8) as u8;
    }

    pub fn get_word(&self, index: u32) -> u32 {
        let index = index as usize;

        let _0 = self.data[index + 0] as u32;
        let _1 = self.data[index + 1] as u32;
        let _2 = self.data[index + 2] as u32;
        let _3 = self.data[index + 3] as u32;

        _0 | (_1 << 8) | (_2 << 16) | (_3 << 24)
    } 

    pub fn set_word(&mut self, index: u32, word: u32) {
        let index = index as usize;

        self.data[index + 0] = word as u8;
        self.data[index + 1] = (word >> 8) as u8;
        self.data[index + 2] = (word >> 16) as u8;
        self.data[index + 3] = (word >> 24) as u8;
    }
}

pub fn from_elf<P: AsRef<Path>>(path: P) -> Result<(Machine, u32), elf::ParseError> {
    let file = elf::File::open_path(path)?;
    let mut data = vec![0; MEM_SIZE];

    for section in file.sections {
        if section.shdr.shtype.0 == 1 { //PT_LOAD
            let addr = section.shdr.addr as usize;
            let size = section.data.len();

            assert!(addr + size <= MEM_SIZE, "Section is too big.");

            for i in 0..size {
                data[addr + i] = section.data[i];
            }
        }
    }

    assert!(file.ehdr.elftype.0 == 2, "File is not executable.");
    assert!(file.ehdr.machine.0 == 8, "File is not MIPS.");
    
    let entry = (file.ehdr.entry / 4) * 4;

    Ok((Machine::new(data), entry as u32))
}
