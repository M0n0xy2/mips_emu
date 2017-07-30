use instruction::Instruction;
use utils;

pub fn decode_instruction(word: u32) -> Instruction {
    if word == 0 { // because NOP is really frequent
        return Instruction::SLL(0, 0, 0);
    }

    let instruction = word >> 26;
    match instruction {
        0b000000 => decode_r_inst(word),
        0b011100 => decode_r2_inst(word),
        0b000001 => decode_branch_comp(word),
        0b000010 => decode_jump(word, Instruction::J),
        0b000011 => decode_jump(word, Instruction::JAL),
        0b000100 => decode_i_sign_extend(word, Instruction::BEQ),
        0b000101 => decode_i_sign_extend(word, Instruction::BNE),
        0b000110 => decode_i_sign_extend(word, |rs, _, imm| Instruction::BLEZ(rs, imm)),
        0b000111 => decode_i_sign_extend(word, |rs, _, imm| Instruction::BGTZ(rs, imm)),
        0b001000 => decode_i_sign_extend(word, Instruction::ADDI),
        0b001001 => decode_i_sign_extend(word, Instruction::ADDIU),
        0b001010 => decode_i_sign_extend(word, Instruction::SLTI),
        0b001011 => decode_i_sign_extend(word, Instruction::SLTIU),
        0b001100 => decode_i_zero_extend(word, Instruction::ANDI),
        0b001101 => decode_i_zero_extend(word, Instruction::ORI),
        0b001110 => decode_i_zero_extend(word, Instruction::XORI),
        0b001111 => decode_i_zero_extend(word, |_, rt, imm| Instruction::LUI(rt, imm)),
        0b100000 => decode_i_sign_extend(word, Instruction::LB),
        0b100001 => decode_i_sign_extend(word, Instruction::LH),
        0b100010 => decode_i_sign_extend(word, Instruction::LWL),
        0b100011 => decode_i_sign_extend(word, Instruction::LW),
        0b100100 => decode_i_sign_extend(word, Instruction::LBU),
        0b100101 => decode_i_sign_extend(word, Instruction::LHU),
        0b100110 => decode_i_sign_extend(word, Instruction::LWR),
        0b101000 => decode_i_sign_extend(word, Instruction::SB),
        0b101001 => decode_i_sign_extend(word, Instruction::SH),
        0b101010 => decode_i_sign_extend(word, Instruction::SWL),
        0b101011 => decode_i_sign_extend(word, Instruction::SW),
        0b101110 => decode_i_sign_extend(word, Instruction::SWR),
        _ => Instruction::Unknown(word),
    }
}

type IZeroExtendConstructor = fn(u32, u32, u32) -> Instruction;
fn decode_i_zero_extend(word: u32, constructor: IZeroExtendConstructor) -> Instruction {
    let rs = (word << 6) >> 27;
    let rt = (word << 11) >> 27;
    let imm = (word << 16) >> 16;

    constructor(rs, rt, imm)
}

type ISignExtendConstructor = fn(u32, u32, i32) -> Instruction;
fn decode_i_sign_extend(word: u32, constructor: ISignExtendConstructor) -> Instruction {
    let rs = (word << 6) >> 27;
    let rt = (word << 11) >> 27;
    let imm = utils::u2i(word << 16) >> 16;

    constructor(rs, rt, imm)
}

fn decode_r_inst(word: u32) -> Instruction {
    let sub_op_code = (word << 26) >> 26;
    match sub_op_code {
        0b000000 => decode_r_shift(word, Instruction::SLL),
        0b000010 => decode_r_shift(word, Instruction::SRL),
        0b000011 => decode_r_shift(word, Instruction::SRA),
        0b000100 => decode_r_no_shift(word, Instruction::SLLV),
        0b000110 => decode_r_no_shift(word, Instruction::SRLV),
        0b000111 => decode_r_no_shift(word, Instruction::SRAV),
        0b001000 => decode_r_no_shift(word, |rs, _, _| Instruction::JR(rs)),
        0b001001 => decode_r_no_shift(word, |rs, _, rd| Instruction::JALR(rs, rd)),
        0b001100 => Instruction::SYSCALL,
        0b010000 => decode_r_no_shift(word, |_, _, rd| Instruction::MFHI(rd)),
        0b010001 => decode_r_no_shift(word, |rs, _, _| Instruction::MTHI(rs)),
        0b010010 => decode_r_no_shift(word, |_, _, rd| Instruction::MFLO(rd)),
        0b010011 => decode_r_no_shift(word, |rs, _, _| Instruction::MTLO(rs)),
        0b011000 => decode_r_div_mul(word, Instruction::MULT),
        0b011001 => decode_r_div_mul(word, Instruction::MULTU),
        0b011010 => decode_r_div_mul(word, Instruction::DIV),
        0b011011 => decode_r_div_mul(word, Instruction::DIVU),
        0b100000 => decode_r_no_shift(word, Instruction::ADD),
        0b100001 => decode_r_no_shift(word, Instruction::ADDU),
        0b100010 => decode_r_no_shift(word, Instruction::SUB),
        0b100011 => decode_r_no_shift(word, Instruction::SUBU),
        0b100100 => decode_r_no_shift(word, Instruction::AND),
        0b100101 => decode_r_no_shift(word, Instruction::OR),
        0b100110 => decode_r_no_shift(word, Instruction::XOR),
        0b100111 => decode_r_no_shift(word, Instruction::NOR),
        0b101010 => decode_r_no_shift(word, Instruction::SLT),
        0b101011 => decode_r_no_shift(word, Instruction::SLTU),
        0b110100 => decode_r_no_shift(word, |rs, rt, _| Instruction::TEQ(rs, rt)),
        _ => Instruction::Unknown(word),
    }
}

fn decode_r2_inst(word: u32) -> Instruction {
    let sub_op_code = (word << 26) >> 26;
    match sub_op_code {
        0b000010 => decode_r_no_shift(word, Instruction::MUL),
        _ => Instruction::Unknown(word),
    }
}

type RNoShiftConstructor = fn(u32, u32, u32) -> Instruction;
fn decode_r_no_shift(word: u32, constructor: RNoShiftConstructor) -> Instruction {
    let rs = (word << 6) >> 27;
    let rt = (word << 11) >> 27;
    let rd = (word << 16) >> 27;

    constructor(rs, rt, rd)
}

type RShiftConstructor = fn(u32, u32, u32) -> Instruction;
fn decode_r_shift(word: u32, constructor: RShiftConstructor) -> Instruction {
    let rt = (word << 11) >> 27;
    let rd = (word << 16) >> 27;
    let shift = (word << 21) >> 27;

    constructor(rt, rd, shift)
}

type RDivMulConstructor = fn(u32, u32) -> Instruction;
fn decode_r_div_mul(word: u32, constructor: RDivMulConstructor) -> Instruction {
    let rs = (word << 6) >> 27;
    let rt = (word << 11) >> 27;

    constructor(rs, rt)
}

fn decode_branch_comp(word: u32) -> Instruction {
    let rs = (word << 6) >> 27;
    let offset = ((word << 16) as i32) >> 16;

    let op = (word << 11) >> 27;
    let constructor = match op {
        0b00000 => Instruction::BLTZ,
        0b00001 => Instruction::BGEZ,
        0b10000 => Instruction::BLTZAL,
        0b10001 => Instruction::BGEZAL,
        _ => return Instruction::Unknown(word),
    };

    constructor(rs, offset)
}

type JumpConstructor = fn(u32) -> Instruction;
fn decode_jump(word: u32, constructor: JumpConstructor) -> Instruction {
    let instr_index = (word << 6) >> 6;
    constructor(instr_index)
}
