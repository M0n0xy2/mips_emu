use std;
use elf;

use memory::Memory;
use instruction::{Instruction, PCOperation};
use utils;

pub struct Cpu {
    registers: [u32; 31],
    pub hi: u32,
    pub lo: u32,
    pub pc: u32,
    npc: u32,
    pub memory: Memory,
    pub state: State,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            registers: [0; 31],
            hi: 0,
            lo: 0,
            pc: 0,
            npc: 4,
            memory: Memory::new(),
            state: State::Paused,
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

    pub fn reset_with_memory(&mut self, memory: Memory) {
        self.registers = [0; 31];
        self.hi = 0;
        self.lo = 0;
        self.pc = 0;
        self.npc = 4;
        self.memory = memory;
    }

    pub fn continue_execution(&mut self, log: bool) {
        if self.state != State::Halted {
            loop {
                self.step(log);
                if self.state != State::Running {
                    break
                }
            }
        }
    }

    pub fn step(&mut self, log: bool) {
        self.state = State::Running;
        let word = self.memory.get_word(self.pc);
        let inst = Instruction::from_word(word);
        
        if log {
            println!("Executing (pc={:#x}): {}", self.pc, inst);
        }

        match inst.apply(self) {
            PCOperation::Offset(value) => self.offset(value),
            PCOperation::JumpReal(index) => self.jump_real(index),
            PCOperation::JumpCompute(index) => self.jump_compute(index),
            PCOperation::Trap(reason) => {
                println!("{}", reason);
                self.offset(4);
            },
            PCOperation::Breakpoint => {
                self.state = State::Paused;
            },
            PCOperation::Exit => {
                self.state = State::Halted;
            }
        }
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

    pub fn load_elf(&mut self, file: elf::File) -> Result<(), String> {
        let mut memory = Memory::new();

        for section in file.sections {
            if section.shdr.shtype.0 == 1 { //PT_LOAD
                let addr = section.shdr.addr as usize;
                let size = section.data.len();

                if addr + size > std::u32::MAX as usize {
                    return Err(format!("Section {} is too big.", section.shdr.name));
                }

                for i in 0..size {
                    memory.set_byte((addr + i) as u32, section.data[i]);
                }
            }
        }

        if file.ehdr.elftype.0 != 2 {
            return Err("File is not executable.".to_string());
        }
        if file.ehdr.machine.0 != 8 {
            return Err("File is not MIPS.".to_string());
        }

        let entry = (file.ehdr.entry / 4) * 4;
        
        self.reset_with_memory(memory);

        self.pc = entry as u32;
        self.npc = self.pc + 4;

        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Running,
    Paused,
    Halted,
}
