use machine::Machine;
use instruction::{Instruction, PCOperation};
use utils;

pub struct Runner {
    pub machine: Machine,
    pub pc: u32,
    npc: u32,
}

impl Runner {
    pub fn new(machine: Machine, entry: u32) -> Runner {
        Runner {
            machine,
            pc: entry,
            npc: entry + 4,
        }
    }

    pub fn run(&mut self) {
        while self.step() {
        }
    }

    pub fn step(&mut self) -> bool { // false => exit
        let word = self.machine.memory.get_word(self.pc);
        let inst = Instruction::from_word(word);
        // println!("Executing (pc={:#x}): {}", self.pc, inst);
        match inst.apply(&mut self.machine, self.pc) {
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
}
