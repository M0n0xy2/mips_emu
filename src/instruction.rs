use machine::Machine;
use decoder;
use executer;

#[derive(Debug, Clone)]
pub enum Instruction {
    Unknown(u32),
    ADD(u32, u32, u32), // rs, rt, rd
    ADDI(u32, u32, i32), // rs, rt, imm
    ADDIU(u32, u32, i32), // rs, rt, imm
    ADDU(u32, u32, u32), // rs, rt, rd
    AND(u32, u32, u32), // rs, rt, rd
    ANDI(u32, u32, u32), // rs, rt, imm
    BEQ(u32, u32, i32), // rs, rt, offset
    BGEZ(u32, i32), // rs, offset
    BGEZAL(u32, i32), // rs, offset
    BGTZ(u32, i32), // rs, offset
    BLEZ(u32, i32), // rs, offset
    BLTZ(u32, i32), // rs, offset
    BLTZAL(u32, i32), // rs, offset
    BNE(u32, u32, i32), // rs, rt, offset
    DIV(u32, u32), // rs, rt
    DIVU(u32, u32), // rs, rt
    J(u32), // instr_index
    JAL(u32), // instr_index
    JALR(u32, u32), // rs, rd
    JR(u32), // rs
    LB(u32, u32, i32), // base, rt, offset
    LBU(u32, u32, i32), // base, rt, offset
    LH(u32, u32, i32), // base, rt, offset
    LHU(u32, u32, i32), // base, rt, offset
    LUI(u32, u32, u32), // rs(unused), rt, imm
    LW(u32, u32, i32), // base, rt, offset
    LWL(u32, u32, i32), // base, rt, offset
    LWR(u32, u32, i32), // base, rt, offset
    MFHI(u32), // rd
    MFLO(u32), // rd
    MTHI(u32), // rs
    MTLO(u32), // rs
    MULT(u32, u32), // rs, rt
    MULTU(u32, u32), // rs, rt
    NOR(u32, u32, u32), // rs, rt, rd
    OR(u32, u32, u32), // rs, rt, rd
    ORI(u32, u32, u32), // rs, rt, imm
    SB(u32, u32, i32), // base, rt, offset
    SH(u32, u32, i32), // base, rt, offset
    SLL(u32, u32, u32), // rt, rd, shift
    SLLV(u32, u32, u32), // rs, rt, rd
    SLT(u32, u32, u32), // rs, rt, rd
    SLTU(u32, u32, u32), // rs, rt, rd
    SLTI(u32, u32, i32), // rs, rt, imm
    SLTIU(u32, u32, i32), // rs, rt, imm
    SRA(u32, u32, u32), // rt, rd, shift
    SRAV(u32, u32, u32), // rs, rt, rd
    SRL(u32, u32, u32), // rt, rd, shift
    SRLV(u32, u32, u32), // rs, rt, rd
    SUB(u32, u32, u32), // rs, rt, rd
    SUBU(u32, u32, u32), // rs, rt, rd
    SW(u32, u32, i32), // base, rt, offset
    SWL(u32, u32, i32), // base, rt, offset
    SWR(u32, u32, i32), // base, rt, offset
    SYSCALL,
    XOR(u32, u32, u32), // rs, rt, rd
    XORI(u32, u32, u32), // rs, rt, imm
}

impl Instruction {
    pub fn apply(&self, machine: &mut Machine) -> PCOperation {
        executer::apply_instruction(self, machine)
    }

    pub fn from_word(word: u32) -> Instruction {
        decoder::decode_instruction(word)
    }
}

pub enum PCOperation {
    Offset(i32),
    JumpReal(u32),
    JumpCompute(u32),
    Trap(String),
    Exit,
}

use std::fmt;
impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Instruction::Unknown(word) => write!(f, "Unknwown {:032b}", word),
            Instruction::ADD(rs, rt, rd) => write!(f, "add ${}, ${}, ${}", rd, rs, rt),
            Instruction::ADDI(rs, rt, imm) => write!(f, "addi ${}, ${}, {}", rt, rs, imm),
            Instruction::ADDIU(rs, rt, imm) => write!(f, "addiu ${}, ${}, {}", rt, rs, imm),
            Instruction::ADDU(rs, rt, rd) => write!(f, "addu ${}, ${}, ${}", rd, rs, rt),
            Instruction::AND(rs, rt, rd) => write!(f, "and ${}, ${}, ${}", rd, rs, rt),
            Instruction::ANDI(rs, rt, imm) => write!(f, "andi ${}, ${}, {}", rt, rs, imm),
            Instruction::BEQ(rs, rt, offset) => write!(f, "beq ${}, ${}, {}", rs, rt, offset),
            Instruction::BGEZ(rs, offset) => write!(f, "bgez ${}, {}", rs, offset),
            Instruction::BGEZAL(rs, offset) => write!(f, "bgezal ${}, {}", rs, offset),
            Instruction::BGTZ(rs, offset) => write!(f, "bgtz ${}, {}", rs, offset),
            Instruction::BLEZ(rs, offset) => write!(f, "blez ${}, {}", rs, offset),
            Instruction::BLTZ(rs, offset) => write!(f, "bltz ${}, {}", rs, offset),
            Instruction::BLTZAL(rs, offset) => write!(f, "bltzal ${}, {}", rs, offset),
            Instruction::BNE(rs, rt, offset) => write!(f, "bne ${}, ${}, {}", rs, rt, offset),
            Instruction::DIV(rs, rt) => write!(f, "div ${}, ${}", rs, rt),
            Instruction::DIVU(rs, rt) => write!(f, "divu ${}, ${}", rs, rt),
            Instruction::J(instr_index) => write!(f, "j {}", instr_index),
            Instruction::JAL(instr_index) => write!(f, "jal {}", instr_index),
            Instruction::JALR(rs, rd) => write!(f, "jalr ${}, ${}", rd, rs),
            Instruction::JR(rs) => write!(f, "jr ${}", rs),
            Instruction::LB(base, rt, offset) => write!(f, "lb ${}, {}(${})", rt, offset, base),
            Instruction::LBU(base, rt, offset) => write!(f, "lbu ${}, {}(${})", rt, offset, base),
            Instruction::LH(base, rt, offset) => write!(f, "lh ${}, {}(${})", rt, offset, base),
            Instruction::LHU(base, rt, offset) => write!(f, "lhu ${}, {}(${})", rt, offset, base),
            Instruction::LUI(_, rt, imm) => write!(f, "lui ${}, {}", rt, imm),
            Instruction::LW(base, rt, offset) => write!(f, "lw ${}, {}(${})", rt, offset, base),
            Instruction::LWL(base, rt, offset) => write!(f, "lwl ${}, {}(${})", rt, offset, base),
            Instruction::LWR(base, rt, offset) => write!(f, "lwr ${}, {}(${})", rt, offset, base),
            Instruction::MFHI(rd) => write!(f, "mfhi ${}", rd),
            Instruction::MFLO(rd) => write!(f, "mflo ${}", rd),
            Instruction::MTHI(rs) => write!(f, "mthi ${}", rs),
            Instruction::MTLO(rs) => write!(f, "mtlo ${}", rs),
            Instruction::MULT(rs, rt) => write!(f, "mult ${}, ${}", rs, rt),
            Instruction::MULTU(rs, rt) => write!(f, "multu ${}, ${}", rs, rt),
            Instruction::NOR(rs, rt, rd) => write!(f, "nor ${}, ${}, ${}", rd, rs, rt),
            Instruction::OR(rs, rt, rd) => write!(f, "or ${}, ${}, ${}", rd, rs, rt),
            Instruction::ORI(rs, rt, imm) => write!(f, "ori ${}, ${}, {}", rt, rs, imm),
            Instruction::SB(base, rt, offset) => write!(f, "sb ${}, {}(${})", rt, offset, base),
            Instruction::SH(base, rt, offset) => write!(f, "sh ${}, {}(${})", rt, offset, base),
            Instruction::SLL(rt, rd, shift) => write!(f, "sll ${}, ${}, {}", rd, rt, shift),
            Instruction::SLLV(rs, rt, rd) => write!(f, "sllv ${}, ${}, ${}", rd, rt, rs),
            Instruction::SLT(rs, rt, rd) => write!(f, "slt ${}, ${}, ${}", rd, rs, rt),
            Instruction::SLTI(rs, rt, imm) => write!(f, "slti ${}, ${}, {}", rs, rt, imm),
            Instruction::SLTIU(rs, rt, imm) => write!(f, "sltiu ${}, ${}, {}", rs, rt, imm),
            Instruction::SLTU(rs, rt, rd) => write!(f, "sltu ${}, ${}, ${}", rd, rs, rt),
            Instruction::SRA(rt, rd, shift) => write!(f, "sra ${}, ${}, {}", rd, rt, shift),
            Instruction::SRAV(rs, rt, rd) => write!(f, "srav ${}, ${}, ${}", rd, rt, rs),
            Instruction::SRL(rt, rd, shift) => write!(f, "srl ${}, ${}, {}", rd, rt, shift),
            Instruction::SRLV(rs, rt, rd) => write!(f, "srlv ${}, ${}, ${}", rd, rt, rs),
            Instruction::SUB(rs, rt, rd) => write!(f, "sub ${}, ${}, ${}", rd, rs, rt),
            Instruction::SUBU(rs, rt, rd) => write!(f, "subu ${}, ${}, ${}", rd, rs, rt),
            Instruction::SW(base, rt, offset) => write!(f, "sw ${}, {}(${})", rt, offset, base),
            Instruction::SWL(base, rt, offset) => write!(f, "swl ${}, {}(${})", rt, offset, base),
            Instruction::SWR(base, rt, offset) => write!(f, "swr ${}, {}(${})", rt, offset, base),
            Instruction::SYSCALL => write!(f, "syscall"),
            Instruction::XOR(rs, rt, rd) => write!(f, "xor ${}, ${}, ${}", rd, rs, rt),
            Instruction::XORI(rs, rt, imm) => write!(f, "xori ${}, ${}, {}", rt, rs, imm),
        }
    }
}
