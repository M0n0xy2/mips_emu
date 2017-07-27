use utils;
use instruction::{Instruction, PCOperation};
use machine::{self, Machine};
use syscall;

macro_rules! check_address_range {
    ($addr:expr) => {
        if $addr as usize >= machine::MEM_SIZE {
            return PCOperation::Trap("Address out of range.".to_string())
        }
    }
}

macro_rules! check_address_aligned_word {
    ($addr:expr) => {
        if ($addr & 0b11) != 0 {
            return PCOperation::Trap("Address unaligned on word boundary.".to_string())
        }
    }
}

macro_rules! check_address_aligned_half_word {
    ($addr:expr) => {
        if ($addr & 0b1) != 0 {
            return PCOperation::Trap("Address unaligned on half-word boundary.".to_string())
        }
    }
}

pub fn apply_instruction(inst: &Instruction, machine: &mut Machine) -> PCOperation {
    let pc = machine.pc;
    match *inst {
        Instruction::Unknown(inst) => {
            panic!("Unknown Instruction (pc= {:#x}). {:032b}", pc, inst);
        },
        Instruction::ADD(rs, rt, rd) => {
            let rs_value = utils::u2i(machine.get_register(rs));
            let rt_value = utils::u2i(machine.get_register(rt));

            if let Some(res) = rs_value.checked_add(rt_value) {
                machine.set_register(rd, utils::i2u(res));
            } else {
                return PCOperation::Trap("Add overflow.".to_string());
            }

            PCOperation::Offset(4)
        },
        Instruction::ADDI(rs, rt, imm) => {
            let rs_value = utils::u2i(machine.get_register(rs));

            if let Some(res) = rs_value.checked_add(imm) {
                machine.set_register(rt, utils::i2u(res));
            } else {
                return PCOperation::Trap("Add overflow.".to_string());
            }

            PCOperation::Offset(4)
        },
        Instruction::ADDIU(rs, rt, imm) => {
            let rs_value = utils::u2i(machine.get_register(rs));
            let result = rs_value.wrapping_add(imm);
            machine.set_register(rt, utils::i2u(result));
            PCOperation::Offset(4)
        },
        Instruction::ADDU(rs, rt, rd) => {
            let rs_value = machine.get_register(rs);
            let rt_value = machine.get_register(rt);
            machine.set_register(rd, rs_value.wrapping_add(rt_value));
            PCOperation::Offset(4)
        },
        Instruction::AND(rs, rt, rd) => {
            let rs_value = machine.get_register(rs);
            let rt_value = machine.get_register(rt);
            machine.set_register(rd, rs_value & rt_value);
            PCOperation::Offset(4)
        },
        Instruction::ANDI(rs, rt, imm) => {
            let rs_value = machine.get_register(rs);
            machine.set_register(rt, rs_value & imm);
            PCOperation::Offset(4)
        },
        Instruction::BEQ(rs, rt, offset) => {
            let rs_value = machine.get_register(rs);
            let rt_value = machine.get_register(rt);

            if rs_value == rt_value {
                PCOperation::Offset(offset << 2)
            } else {
                PCOperation::Offset(4)
            }
        },
        Instruction::BGEZ(rs, offset) => {
            let rs_value = utils::u2i(machine.get_register(rs));
            
            if rs_value >= 0 {
                PCOperation::Offset(offset << 2)
            } else {
                PCOperation::Offset(4)
            }
        },
        Instruction::BGEZAL(rs, offset) => {
            machine.set_register(31, pc + 8);
            let rs_value = utils::u2i(machine.get_register(rs));

            if rs_value >= 0 {
                PCOperation::Offset(offset << 2)
            } else {
                PCOperation::Offset(4)
            }
        },
        Instruction::BGTZ(rs, offset) => {
            let rs_value = utils::u2i(machine.get_register(rs));
            
            if rs_value > 0 {
                PCOperation::Offset(offset << 2)
            } else {
                PCOperation::Offset(4)
            }
        },
        Instruction::BLEZ(rs, offset) => {
            let rs_value = utils::u2i(machine.get_register(rs));

            if rs_value <= 0 {
                PCOperation::Offset(offset << 2)
            } else {
                PCOperation::Offset(4)
            }

        },
        Instruction::BLTZ(rs, offset) => {
            let rs_value = utils::u2i(machine.get_register(rs));

            if rs_value < 0 {
                PCOperation::Offset(offset << 2)
            } else {
                PCOperation::Offset(4)
            }
        },
        Instruction::BLTZAL(rs, offset) => {
            machine.set_register(31, pc + 8);
            let rs_value = utils::u2i(machine.get_register(rs));

            if rs_value < 0 {
                PCOperation::Offset(offset << 2)
            } else {
                PCOperation::Offset(4)
            }
        },
        Instruction::BNE(rs, rt, offset) => {
            let rs_value = machine.get_register(rs);
            let rt_value = machine.get_register(rt);

            if rs_value != rt_value {
                PCOperation::Offset(offset << 2)
            } else {
                PCOperation::Offset(4)
            }
        },
        Instruction::DIV(rs, rt) => {
            let rs_value = utils::u2i(machine.get_register(rs));
            let rt_value = utils::u2i(machine.get_register(rt));

            let q = rs_value / rt_value;
            let r = rs_value % rt_value;

            machine.lo = utils::i2u(q);
            machine.hi = utils::i2u(r);

            PCOperation::Offset(4)

        },
        Instruction::DIVU(rs, rt) => {
            let rs_value = machine.get_register(rs);
            let rt_value = machine.get_register(rt);

            let q = rs_value / rt_value;
            let r = rs_value % rt_value;

            machine.lo = q;
            machine.hi = r;

            PCOperation::Offset(4)
        },
        Instruction::J(instr_index) => {
            PCOperation::JumpCompute(instr_index)
        },
        Instruction::JAL(instr_index) => {
            machine.set_register(31, pc + 8);
            PCOperation::JumpCompute(instr_index)
        },
        Instruction::JALR(rs, rd) => {
            machine.set_register(rd, pc + 8);
            let addr = machine.get_register(rs);
            PCOperation::JumpReal(addr)
        },
        Instruction::JR(rs) => {
            let addr = machine.get_register(rs);
            PCOperation::JumpReal(addr)
        },
        Instruction::LB(base, rt, offset) => {
            let addr = utils::offset_addr(machine.get_register(base), offset);
            check_address_range!(addr);

            let byte = machine.memory.get_byte(addr) as i8;
            machine.set_register(rt, utils::i2u(byte as i32));
            PCOperation::Offset(4)
        },
        Instruction::LBU(base, rt, offset) => {
            let addr = utils::offset_addr(machine.get_register(base), offset);
            check_address_range!(addr);

            let byte = machine.memory.get_byte(addr);
            machine.set_register(rt, byte as u32);
            PCOperation::Offset(4)
        },
        Instruction::LH(base, rt, offset) => {
            let addr = utils::offset_addr(machine.get_register(base), offset);
            check_address_range!(addr);
            check_address_aligned_half_word!(addr);

            let half = machine.memory.get_half_word(addr) as i16;
            machine.set_register(rt, utils::i2u(half as i32));
            PCOperation::Offset(4)
        },
        Instruction::LHU(base, rt, offset) => {
            let addr = utils::offset_addr(machine.get_register(base), offset);
            check_address_range!(addr);
            check_address_aligned_half_word!(addr);

            let half = machine.memory.get_half_word(addr);
            machine.set_register(rt, half as u32);
            PCOperation::Offset(4)
        },
        Instruction::LUI(_, rt, imm) => {
            machine.set_register(rt, imm << 16);
            PCOperation::Offset(4)
        },
        Instruction::LW(base, rt, offset) => {
            let addr = utils::offset_addr(machine.get_register(base), offset);

            check_address_range!(addr);
            check_address_aligned_word!(addr);
            
            let word = machine.memory.get_word(addr);
            machine.set_register(rt, word);
            PCOperation::Offset(4)
        },
        Instruction::LWL(base, rt, offset) => {
            let rt_value = machine.get_register(rt);
            let addr = utils::offset_addr(machine.get_register(base), offset);

            check_address_range!(addr);
            
            let unaligned_offset = addr & 0b11;

            let mem_part = machine.memory.get_word(addr - unaligned_offset) << (8 * (3 - unaligned_offset));
            let reg_part = if unaligned_offset != 3 {
                rt_value & (0xFFFFFFFFu32 >> (8 * (unaligned_offset + 1)))
            } else {
                0
            };
            let result = mem_part | reg_part;

            machine.set_register(rt, result);
            PCOperation::Offset(4)
        },
        Instruction::LWR(base, rt, offset) => {
            let rt_value = machine.get_register(rt);
            let addr = utils::offset_addr(machine.get_register(base), offset);

            check_address_range!(addr);

            let unaligned_offset = addr & 0b11;

            let mem_part = machine.memory.get_word(addr - unaligned_offset) >> 8 * unaligned_offset;
            let reg_part = if unaligned_offset != 0 {
                rt_value & (0xFFFFFFFFu32 << (8 * (4 - unaligned_offset)))
            } else {
                0
            };
            let result = mem_part | reg_part;

            machine.set_register(rt, result);
            PCOperation::Offset(4)
        },
        Instruction::MFHI(rd) => {
            let value = machine.hi;
            machine.set_register(rd, value);
            PCOperation::Offset(4)
        },
        Instruction::MFLO(rd) => {
            let value = machine.lo;
            machine.set_register(rd, value);
            PCOperation::Offset(4)
        },
        Instruction::MTHI(rs) => {
            machine.hi = machine.get_register(rs);
            PCOperation::Offset(4)
        },
        Instruction::MTLO(rs) => {
            machine.lo = machine.get_register(rs);
            PCOperation::Offset(4)
        },
        Instruction::MULT(rs, rt) => {
            let rs_value = utils::u2i(machine.get_register(rs)) as i64;
            let rt_value = utils::u2i(machine.get_register(rt)) as i64;

            let result = rs_value * rt_value;
            machine.lo = utils::i2u(result as i32);
            machine.hi = utils::i2u((result >> 32) as i32);
            PCOperation::Offset(4)
        },
        Instruction::MULTU(rs, rt) => {
            let rs_value = machine.get_register(rs) as u64;
            let rt_value = machine.get_register(rt) as u64;

            let result = rs_value * rt_value;
            machine.lo = result as u32;
            machine.hi = (result >> 32) as u32;
            PCOperation::Offset(4)
        },
        Instruction::NOR(rs, rt, rd) => {
            let rs_value = machine.get_register(rs);
            let rt_value = machine.get_register(rt);
            machine.set_register(rd, !(rs_value | rt_value));
            PCOperation::Offset(4)
        },
        Instruction::OR(rs, rt, rd) => {
            let rs_value = machine.get_register(rs);
            let rt_value = machine.get_register(rt);
            machine.set_register(rd, rs_value | rt_value);
            PCOperation::Offset(4)
        },
        Instruction::ORI(rs, rt, imm) => {
            let rs_value = machine.get_register(rs);
            machine.set_register(rt, rs_value | imm);
            PCOperation::Offset(4)
        },
        Instruction::SB(base, rt, offset) => {
            let word = machine.get_register(rt);
            let byte = word as u8;

            let addr = utils::offset_addr(machine.get_register(base), offset);
            check_address_range!(addr);

            machine.memory.set_byte(addr, byte);
            PCOperation::Offset(4)
        },
        Instruction::SH(base, rt, offset) => {
            let word = machine.get_register(rt);
            let half = word as u16;

            let addr = utils::offset_addr(machine.get_register(base), offset);
            check_address_range!(addr);

            machine.memory.set_half_word(addr, half);
            PCOperation::Offset(4)
        },
        Instruction::SLL(rt, rd, shift) => {
            let rt_value = machine.get_register(rt);
            machine.set_register(rd, rt_value << shift);
            PCOperation::Offset(4)
        },
        Instruction::SLLV(rs, rt, rd) => {
            let rt_value = machine.get_register(rt);
            let shift = (machine.get_register(rs) << 27) >> 27;
            machine.set_register(rd, rt_value << shift);
            PCOperation::Offset(4)
        },
        Instruction::SLT(rs, rt, rd) => {
            let rs_value = utils::u2i(machine.get_register(rs));
            let rt_value = utils::u2i(machine.get_register(rt));
            let res = if rs_value < rt_value { 1 } else { 0 };
            machine.set_register(rd, res);
            PCOperation::Offset(4)
        },
        Instruction::SLTI(rs, rt, imm) => {
            let rs_value = utils::u2i(machine.get_register(rs));
            let res = if rs_value < imm { 1 } else { 0 };
            machine.set_register(rt, res);
            PCOperation::Offset(4)
        },
        Instruction::SLTIU(rs, rt, imm) => {
            let rs_value = machine.get_register(rs);
            let imm = imm as u32;
            let res = if rs_value < imm { 1 } else { 0 };
            machine.set_register(rt, res);
            PCOperation::Offset(4)
        },
        Instruction::SLTU(rs, rt, rd) => {
            let rs_value = machine.get_register(rs);
            let rt_value = machine.get_register(rt);
            machine.set_register(rd, if rs_value < rt_value { 1 } else { 0 });
            PCOperation::Offset(4)
        },
        Instruction::SRA(rt, rd, shift) => {
            let rt_value = machine.get_register(rt);
            let result = utils::u2i(rt_value) >> shift;
            machine.set_register(rd, utils::i2u(result));
            PCOperation::Offset(4)
        },
        Instruction::SRAV(rs, rt, rd) => {
            let rt_value = machine.get_register(rt);
            let shift = (machine.get_register(rs) << 27) >> 27;

            let result = utils::u2i(rt_value) >> shift;
            machine.set_register(rd, utils::i2u(result));
            PCOperation::Offset(4)
        },
        Instruction::SRL(rt, rd, shift) => {
            let rt_value = machine.get_register(rt);
            machine.set_register(rd, rt_value >> shift);
            PCOperation::Offset(4)
        },
        Instruction::SRLV(rs, rt, rd) => {
            let rt_value = machine.get_register(rt);
            let shift = (machine.get_register(rs) << 27) >> 27;
            machine.set_register(rd, rt_value >> shift);
            PCOperation::Offset(4)
        },
        Instruction::SUB(rs, rt, rd) => {
            let rs_value = utils::u2i(machine.get_register(rs));
            let rt_value = utils::u2i(machine.get_register(rt));

            if let Some(res) = rs_value.checked_sub(rt_value) {
                machine.set_register(rd, utils::i2u(res));
            } else {
                return PCOperation::Trap("Sub underflow.".to_string());
            }

            PCOperation::Offset(4)
        },
        Instruction::SUBU(rs, rt, rd) => {
            let rs_value = machine.get_register(rs);
            let rt_value = machine.get_register(rt);
            machine.set_register(rd, rs_value.wrapping_sub(rt_value));
            PCOperation::Offset(4)
        },
        Instruction::SW(base, rt, offset) => {
            let addr = utils::offset_addr(machine.get_register(base), offset);
            check_address_range!(addr);

            let word = machine.get_register(rt);
            machine.memory.set_word(addr, word);

            PCOperation::Offset(4)
        },
        Instruction::SWL(base, rt, offset) => {
            let rt_value = machine.get_register(rt);
            let addr = utils::offset_addr(machine.get_register(base), offset);
            check_address_range!(addr);
            
            let unaligned_offset = addr & 0b11;
            let mem_part = if unaligned_offset != 3 {
                machine.memory.get_word(addr - unaligned_offset)
                    & (0xFFFFFFFFu32 << (8 * (unaligned_offset + 1)))
            } else {
                0
            };
            let reg_part = rt_value >> (8 * (3 - unaligned_offset));

            machine.memory.set_word(addr - unaligned_offset, mem_part | reg_part);
            PCOperation::Offset(4)
        },
        Instruction::SWR(base, rt, offset) => {
            let rt_value = machine.get_register(rt);
            let addr = utils::offset_addr(machine.get_register(base), offset);
            check_address_range!(addr);

            let unaligned_offset = addr & 0b11;
            let mem_part = if unaligned_offset != 0 {
                machine.memory.get_word(addr - unaligned_offset)
                    & (0xFFFFFFFFu32 >> 8 * (4 - unaligned_offset))
            } else {
                0
            };
            let reg_part = rt_value << 8 * unaligned_offset;
            
            machine.memory.set_word(addr - unaligned_offset, mem_part | reg_part);
            PCOperation::Offset(4)
        },
        Instruction::SYSCALL => syscall::call_syscall(machine),
        Instruction::XOR(rs, rt, rd) => {
            let rs_value = machine.get_register(rs);
            let rt_value = machine.get_register(rt);
            machine.set_register(rd, rs_value ^ rt_value);
            PCOperation::Offset(4)
        },
        Instruction::XORI(rs, rt, imm) => {
            let rs_value = machine.get_register(rs);
            machine.set_register(rt, rs_value ^ imm);
            PCOperation::Offset(4)
        },
    }
}
