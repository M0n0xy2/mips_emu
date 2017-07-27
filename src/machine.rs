use std::path::Path;
use elf;

use memory::Memory;

pub const MEM_SIZE: usize = 1 << 24;

#[derive(Debug, Clone)]
pub struct Machine {
    registers: [u32; 31],
    pub hi: u32,
    pub lo: u32,
    pub memory: Memory,
}

impl Machine {
    pub fn new(data: Vec<u8>) -> Machine {
        let memory = Memory {
            data
        };

        Machine {
            registers: [0; 31],
            hi: 0,
            lo: 0,
            memory,
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
