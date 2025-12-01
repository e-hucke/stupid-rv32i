pub struct CPU {
    pub regs: [u32; 32],
    pub pc: u32,
}

impl CPU {
    pub fn interpret(&mut self, instruction: u32) -> () {
        let op_code = (instruction & 0x7F) as u8;

        match op_code {
            0b_0110011 => {
                let rd = ((instruction >> 7) & 0x1F) as u8;
                let funct3 = (instruction >> 12) & 0x7;
                let rs1 = ((instruction >> 15) & 0x1F) as u8;
                let rs2 = ((instruction >> 20) & 0x1F) as u8;
                let funct7 = (instruction >> 25) & 0x7F;
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
            0b_0010011 => {
                let rd = ((instruction >> 7) & 0x1F) as u8;
                let funct3 = (instruction >> 12) & 0x7;
                let rs1 = ((instruction >> 15) & 0x1F) as u8;
                let imm = ((instruction as i32) >> 20) as u32;
                let imm_lower = (instruction >> 20) & 0x1F;
                let imm_upper = (instruction >> 25) & 0x7F;
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
            0b_0000011 => {
                let rd = ((instruction >> 7) & 0x1F) as u8;
                let funct3 = (instruction >> 12) & 0x7;
                let rs1 = ((instruction >> 15) & 0x1F) as u8;
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
            0b_1100111 => {
                let funct3 = (instruction >> 12) & 0x7;
                match funct3 {
                    0x0 => todo!("jalr"),
                    _ => panic!(),
                }
            }
            0b_1110011 => {
                let funct3 = (instruction >> 12) & 0x7;
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
            0b_0100011 => {
                let funct3 = (instruction >> 12) & 0x7;
                let rs1 = ((instruction >> 15) & 0x1F) as u8;
                let rs2 = ((instruction >> 20) & 0x1F) as u8;
                let imm = ((instruction >> 7) & 0x1F) | (((instruction as i32 >> 25) as u32) << 5);
                match funct3 {
                    0x0 => todo!("sb"),
                    0x1 => todo!("sh"),
                    0x2 => todo!("sw"),
                    _ => panic!(),
                }
            }
            0b1100011 => {
                let funct3 = (instruction >> 12) & 0x7;
                let rs1 = ((instruction >> 15) & 0x1F) as u8;
                let rs2 = ((instruction >> 20) & 0x1F) as u8;
                // todo wtf how do i decode imm here (b type)
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
            0b_1101111 => {
                let rd = ((instruction >> 7) & 0x1F) as u8;
                // todo wtf how do i decode imm here (j type)
            }
            0b_0110111 => {
                let imm = instruction & 0xFFFFF000;
                todo!("lui")
            }
            0b_0010111 => {
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
