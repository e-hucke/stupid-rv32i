pub fn bits_from_to(bits: u32, upper: u32, lower: u32) -> u32 {
    (bits << (31 - upper)) >> (lower + (31 - upper))
}

fn funct3(instruction: u32) -> u32 {
    bits_from_to(instruction, 14, 12)
}

fn rd(instruction: u32) -> u8 {
    bits_from_to(instruction, 11, 7) as u8
}

pub fn s_type(instruction: u32) -> (u32, u8, u8, u32) {
    let funct3 = funct3(instruction);
    let rs1 = rs1(instruction);
    let rs2 = rs2(instruction);
    let imm = bits_from_to(instruction, 11, 7) | (((instruction as i32 >> 25) as u32) << 5);
    (funct3, rs1, rs2, imm)
}

pub fn i_type(instruction: u32) -> (u8, u32, u8, u32) {
    let rd = rd(instruction);
    let funct3 = funct3(instruction);
    let rs1 = rs1(instruction);
    let imm = ((instruction as i32) >> 20) as u32;
    (rd, funct3, rs1, imm)
}

pub fn j_type(instruction: u32) -> (u8, u32) {
    let rd = rd(instruction);
    let imm = 0xFFFFFFFE
        & (((instruction as i32 >> 11) as u32 & 0xFFF00000)
            | (instruction & 0xFF000)
            | ((instruction >> 9) & 0x800)
            | ((instruction >> 20) & 0x7FE));
    (rd, imm)
}

pub fn b_type(instruction: u32) -> (u32, u8, u8, u32) {
    let funct3 = funct3(instruction);
    let rs1 = rs1(instruction);
    let rs2 = rs2(instruction);
    let imm = 0xFFFFFFFE
        & ((((instruction as i32 >> 19) as u32) & 0xFFFFF000)
            | ((instruction << 4) & 0x800)
            | ((instruction >> 20) & 0x7E0)
            | ((instruction >> 7) & 0x1E));
    (funct3, rs1, rs2, imm)
}

pub fn op_code(instruction: u32) -> u8 {
    bits_from_to(instruction, 6, 0) as u8
}

pub fn u_type(instruction: u32) -> (u8, u32) {
    let rd = rd(instruction);
    let imm = instruction & 0xFFFFF000;
    (rd, imm)
}

pub fn r_type(instruction: u32) -> (u8, u32, u8, u8, u32) {
    let rd = rd(instruction);
    let funct3 = funct3(instruction);
    let rs1 = rs1(instruction);
    let rs2 = rs2(instruction);
    let funct7 = bits_from_to(instruction, 31, 25);
    (rd, funct3, rs1, rs2, funct7)
}

pub fn is_compressed_instr(instruction: u16) -> bool {
    instruction & 0x2 != 0x2
}

fn rs1(instruction: u32) -> u8 {
    bits_from_to(instruction, 19, 15) as u8
}

fn rs2(instruction: u32) -> u8 {
    bits_from_to(instruction, 24, 20) as u8
}
