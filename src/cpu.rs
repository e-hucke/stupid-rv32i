pub struct CPU {
    pub regs: [u32; 32],
    pub pc: u32,
}

fn bits_from_to(bits: u32, upper: u32, lower: u32) -> u32 {
    (bits << (31 - upper)) >> (lower + (31 - upper))
}

fn funct3(instruction: u32) -> u32 {
    bits_from_to(instruction, 14, 12)
}

fn rd(instruction: u32) -> u8 {
    bits_from_to(instruction, 11, 7) as u8
}

fn rs1(instruction: u32) -> u8 {
    bits_from_to(instruction, 19, 15) as u8
}

fn rs2(instruction: u32) -> u8 {
    bits_from_to(instruction, 24, 20) as u8
}

impl CPU {
    pub fn interpret(&mut self, instruction: u32) -> () {
        let op_code = bits_from_to(instruction, 6, 0) as u8;

        match op_code {
            // r type
            0b_0110011 => {
                let rd = rd(instruction);
                let funct3 = funct3(instruction);
                let rs1 = rs1(instruction);
                let rs2 = rs2(instruction);
                let funct7 = bits_from_to(instruction, 31, 25);
                match (funct3, funct7) {
                    (0x0, 0x00) => self.add(rd, rs1, rs2),
                    (0x0, 0x20) => self.sub(rd, rs1, rs2),
                    (0x4, 0x00) => self.xor(rd, rs1, rs2),
                    (0x6, 0x00) => self.or(rd, rs1, rs2),
                    (0x7, 0x00) => self.and(rd, rs1, rs2),
                    (0x1, 0x00) => self.sll(rd, rs1, rs2),
                    (0x5, 0x00) => self.srl(rd, rs1, rs2),
                    (0x5, 0x20) => self.sra(rd, rs1, rs2),
                    (0x2, 0x00) => self.slt(rd, rs1, rs2),
                    (0x3, 0x00) => self.sltu(rd, rs1, rs2),
                    (_, _) => panic!(),
                }
            }
            // i type
            0b_0010011 => {
                let rd = rd(instruction);
                let funct3 = funct3(instruction);
                let rs1 = rs1(instruction);
                let imm = ((instruction as i32) >> 20) as u32;
                let imm_lower = bits_from_to(instruction, 24, 20);
                let imm_upper = bits_from_to(instruction, 31, 25);
                match funct3 {
                    0x0 => todo!("addi"),
                    0x4 => todo!("xori"),
                    0x6 => todo!("ori"),
                    0x7 => todo!("andi"),
                    0x2 => todo!("slti"),
                    0x3 => todo!("sltiu"),
                    0x1 => match imm_upper {
                        0x00 => todo!("slli"),
                        _ => panic!(),
                    },
                    0x5 => match imm_upper {
                        0x00 => todo!("srli"),
                        0x20 => todo!("srai"),
                        _ => panic!(),
                    },
                    _ => panic!(),
                }
            }
            // i type
            0b_0000011 => {
                let rd = rd(instruction);
                let funct3 = funct3(instruction);
                let rs1 = rs1(instruction);
                let imm = ((instruction as i32) >> 20) as u32;
                match funct3 {
                    0x0 => todo!("lb"),
                    0x1 => todo!("lh"),
                    0x2 => todo!("lw"),
                    0x4 => todo!("lbu"),
                    0x5 => todo!("lhu"),
                    _ => panic!(),
                }
            }
            // i type
            0b_1100111 => {
                let funct3 = funct3(instruction);
                match funct3 {
                    0x0 => todo!("jalr"),
                    _ => panic!(),
                }
            }
            // i type
            0b_1110011 => {
                let funct3 = funct3(instruction);
                let imm = ((instruction as i32) >> 20) as u32;
                match funct3 {
                    0x0 => match imm {
                        0x0 => todo!("ecall"),
                        0x1 => todo!("ebreak"),
                        _ => panic!(),
                    },
                    _ => panic!(),
                }
            }
            // s type
            0b_0100011 => {
                let funct3 = funct3(instruction);
                let rs1 = rs1(instruction);
                let rs2 = rs2(instruction);
                let imm =
                    bits_from_to(instruction, 11, 7) | (((instruction as i32 >> 25) as u32) << 5);
                match funct3 {
                    0x0 => todo!("sb"),
                    0x1 => todo!("sh"),
                    0x2 => todo!("sw"),
                    _ => panic!(),
                }
            }
            // b type
            0b_1100011 => {
                let funct3 = funct3(instruction);
                let rs1 = rs1(instruction);
                let rs2 = rs2(instruction);
                let imm = 0xFFFFFFFE
                    & ((((instruction as i32 >> 19) as u32) & 0xFFFFF000)
                        | ((instruction << 4) & 0x800)
                        | ((instruction >> 20) & 0x7E0)
                        | ((instruction >> 7) & 0x1E));
                match funct3 {
                    0x0 => todo!("beq"),
                    0x1 => todo!("bne"),
                    0x4 => todo!("blt"),
                    0x5 => todo!("bge"),
                    0x6 => todo!("bltu"),
                    0x7 => todo!("bgeu"),
                    _ => panic!(),
                }
            }
            // j type
            0b_1101111 => {
                let rd = rd(instruction);
                let imm = 0xFFFFFFFE
                    & (((instruction as i32 >> 11) as u32 & 0xFFF00000)
                        | (instruction & 0xFF000)
                        | ((instruction >> 9) & 0x800)
                        | ((instruction >> 20) & 0x7FE));
                todo!("jal")
            }
            // u type
            0b_0110111 => {
                let rd = rd(instruction);
                let imm = instruction & 0xFFFFF000;
                todo!("lui")
            }
            // u type
            0b_0010111 => {
                let rd = rd(instruction);
                let imm = instruction & 0xFFFFF000;
                todo!("auipc")
            }
            _ => panic!("fick dich doch alter"),
        }
    }

    fn slt(&mut self, rd: u8, rs1: u8, rs2: u8) {
        self.write_reg(
            rd,
            if (self.read_reg(rs1) as i32) < (self.read_reg(rs2) as i32) {
                1
            } else {
                0
            },
        )
    }

    fn sltu(&mut self, rd: u8, rs1: u8, rs2: u8) {
        self.write_reg(
            rd,
            if self.read_reg(rs1) < self.read_reg(rs2) {
                1
            } else {
                0
            },
        )
    }
    fn srl(&mut self, rd: u8, rs1: u8, rs2: u8) {
        self.write_reg(rd, self.read_reg(rs1) >> (self.read_reg(rs2) & 0x1F))
    }

    fn write_reg(&mut self, reg: u8, val: u32) {
        if reg != 0 {
            self.regs[reg as usize] = val;
        }
    }

    fn sra(&mut self, rd: u8, rs1: u8, rs2: u8) {
        self.write_reg(
            rd,
            ((self.read_reg(rs1) as i32) >> (self.read_reg(rs2) & 0x1F)) as u32,
        )
    }

    fn read_reg(&self, reg: u8) -> u32 {
        if reg == 0 { 0 } else { self.regs[reg as usize] }
    }

    fn xor(&mut self, rd: u8, rs1: u8, rs2: u8) -> () {
        self.write_reg(rd, self.read_reg(rs1) ^ self.read_reg(rs2));
    }
    fn sub(&mut self, rd: u8, rs1: u8, rs2: u8) {
        self.write_reg(rd, self.read_reg(rs1).wrapping_sub(self.read_reg(rs2)));
    }

    fn add(&mut self, rd: u8, rs_1: u8, rs_2: u8) {
        self.write_reg(rd, self.read_reg(rs_1).wrapping_add(self.read_reg(rs_2)));
    }

    fn sll(&mut self, rd: u8, rs1: u8, rs2: u8) {
        self.write_reg(rd, self.read_reg(rs1) << (self.read_reg(rs2) & 0x1F));
    }
    fn or(&mut self, rd: u8, rs1: u8, rs2: u8) {
        self.write_reg(rd, self.read_reg(rs1) | self.read_reg(rs2));
    }

    fn and(&mut self, rd: u8, rs1: u8, rs2: u8) {
        self.write_reg(rd, self.read_reg(rs1) & self.read_reg(rs2));
    }
}
