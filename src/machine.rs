use std::path::Path;
use elf;

use memory::Memory;
use instruction::{Instruction, PCOperation};
use utils;

pub const MEM_SIZE: usize = 1 << 24;

#[derive(Debug, Clone)]
pub struct Machine {
    registers: [u32; 31],
    pub hi: u32,
    pub lo: u32,
    pub pc: u32,
    npc: u32,
    pub memory: Memory,
}

impl Machine {
    pub fn new() -> Machine {
        Machine {
            registers: [0; 31],
            hi: 0,
            lo: 0,
            pc: 0,
            npc: 4,
            memory: Memory::new(MEM_SIZE),
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

    pub fn reset(&mut self) {
        self.registers = [0; 31];
        self.hi = 0;
        self.lo = 0;
        self.pc = 0;
        self.npc = 4;
        self.memory = Memory::new(MEM_SIZE);
    }

    pub fn run(&mut self) {
        while self.step() {
        }
    }

    pub fn step(&mut self) -> bool { // false => exit
        let word = self.memory.get_word(self.pc);
        let inst = Instruction::from_word(word);
        // println!("Executing (pc={:#x}): {}", self.pc, inst);
        match inst.apply(self) {
            PCOperation::Offset(value) => self.offset(value),
            PCOperation::JumpReal(index) => self.jump_real(index),
            PCOperation::JumpCompute(index) => self.jump_compute(index),
            PCOperation::Trap(reason) => {
                println!("{}", reason);
                self.offset(4);
            }
            PCOperation::Exit => return false,
        }

        true
    }

    pub fn offset(&mut self, offset: i32) {
        self.pc = self.npc;
        self.npc = utils::offset_addr(self.npc, offset);
    }

    pub fn jump_real(&mut self, index: u32) {
        self.pc = self.npc;
        self.npc = index;
    }

    pub fn jump_compute(&mut self, index: u32) {
        self.pc = self.npc;

        let upper = (self.npc >> 28) << 28;
        let lower = (index << 6) >> 4;
        self.npc = upper | lower;
    }

    pub fn load_elf<P: AsRef<Path>>(&mut self, path: P) -> Result<(), elf::ParseError> {
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
        
        self.reset();
        self.memory = Memory::from_data(data);

        self.pc = entry as u32;
        self.npc = self.pc + 4;

        Ok(())
    }
}
