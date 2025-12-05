use crate::decode;
use crate::decode::b_type;
use crate::memory::Memory;

pub struct CPU {
    pub regs: [u32; 32],
    pub pc: u32,
    pub mem: Memory,
}

impl Default for CPU {
    fn default() -> Self {
        CPU {
            regs: [0; 32],
            pc: 0,
            mem: Memory::default(),
        }
    }
}

impl CPU {
    // this returns a bool indicating if we jumped in order to decide if we need to increase the pc
    // no idea how we could do this in a cleaner way D:
    pub fn interpret(&mut self, instruction: u32) -> bool {
        let op_code = decode::op_code(instruction);
        match op_code {
            0b_0110011 => self.interpret_0110011(instruction),
            0b_0010011 => self.interpret_0010011(instruction),
            0b_0000011 => self.interpret_0000011(instruction),
            0b_1100111 => self.interpret_1100111(instruction),
            0b_1110011 => self.interpret_1110011(instruction),
            0b_0100011 => self.interpret_0100011(instruction),
            0b_1100011 => self.interpret_1100011(instruction),
            0b_1101111 => self.interpret_1101111(instruction),
            0b_0110111 => self.interpret_0110111(instruction),
            0b_0010111 => self.interpret_0010111(instruction),
            _ => panic!("fick dich doch alter"),
        }
    }

    pub fn interpret_c(&mut self, instruction: u16) -> bool {
        todo!("compressed extension")
    }

    fn interpret_0110011(&mut self, instruction: u32) -> bool {
        let (rd, funct3, rs1, rs2, funct7) = decode::r_type(instruction);
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
            (0x0, 0x01) => todo!("mul"),
            (0x1, 0x01) => todo!("mulh"),
            (0x2, 0x01) => todo!("mulsu"),
            (0x3, 0x01) => todo!("mulu"),
            (0x4, 0x01) => todo!("div"),
            (0x5, 0x01) => todo!("divu"),
            (0x6, 0x01) => todo!("rem"),
            (0x7, 0x01) => todo!("remu"),
            (_, _) => panic!(),
        };
        false
    }

    fn interpret_0010011(&mut self, instruction: u32) -> bool {
        let (rd, funct3, rs1, imm) = decode::i_type(instruction);
        let imm_upper = decode::bits_from_to(instruction, 31, 25);
        match funct3 {
            0x0 => self.addi(rd, rs1, imm),
            0x4 => self.xori(rd, rs1, imm),
            0x6 => self.ori(rd, rs1, imm),
            0x7 => self.andi(rd, rs1, imm),
            0x2 => self.slti(rd, rs1, imm),
            0x3 => self.sltiu(rd, rs1, imm),
            0x1 => match imm_upper {
                0x00 => self.slli(rd, rs1, imm),
                _ => panic!(),
            },
            0x5 => match imm_upper {
                0x00 => self.srli(rd, rs1, imm),
                0x20 => self.srai(rd, rs1, imm),
                _ => panic!(),
            },
            _ => panic!(),
        };
        false
    }

    fn interpret_0000011(&mut self, instruction: u32) -> bool {
        let (rd, funct3, rs1, imm) = decode::i_type(instruction);
        match funct3 {
            0x0 => self.lb(rd, rs1, imm),
            0x1 => self.lh(rd, rs1, imm),
            0x2 => self.lw(rd, rs1, imm),
            0x4 => self.lbu(rd, rs1, imm),
            0x5 => self.lhu(rd, rs1, imm),
            _ => panic!(),
        };
        false
    }

    fn interpret_1100111(&mut self, instruction: u32) -> bool {
        let (rd, funct3, rs1, imm) = decode::i_type(instruction);
        match funct3 {
            0x0 => self.jalr(rd, rs1, imm),
            _ => panic!(),
        };
        true
    }

    fn interpret_1110011(&mut self, instruction: u32) -> bool {
        let (_, funct3, _, imm) = decode::i_type(instruction);
        match funct3 {
            0x0 => match imm {
                // tbh no idea wtf these even do. todo: read the spec lol
                0x0 => todo!("ecall"),
                0x1 => todo!("ebreak"),
                _ => panic!(),
            },
            _ => panic!(),
        }
        false // ????
    }

    fn interpret_0100011(&mut self, instruction: u32) -> bool {
        let (funct3, rs1, rs2, imm) = decode::s_type(instruction);
        match funct3 {
            0x0 => self.sb(rs1, rs2, imm),
            0x1 => self.sh(rs1, rs2, imm),
            0x2 => self.sw(rs1, rs2, imm),
            _ => panic!(),
        }
        false
    }

    fn interpret_1100011(&mut self, instruction: u32) -> bool {
        let (funct3, rs1, rs2, imm) = b_type(instruction);
        match funct3 {
            0x0 => self.beq(rs1, rs2, imm),
            0x1 => self.bne(rs1, rs2, imm),
            0x4 => self.blt(rs1, rs2, imm),
            0x5 => self.bge(rs1, rs2, imm),
            0x6 => self.bltu(rs1, rs2, imm),
            0x7 => self.bgeu(rs1, rs2, imm),
            _ => panic!(),
        }
    }

    fn interpret_1101111(&mut self, instruction: u32) -> bool {
        let (rd, imm) = decode::j_type(instruction);
        self.jal(rd, imm);
        true
    }

    fn interpret_0110111(&mut self, instruction: u32) -> bool {
        let (rd, imm) = decode::u_type(instruction);
        self.lui(rd, imm);
        false
    }

    fn interpret_0010111(&mut self, instruction: u32) -> bool {
        let (rd, imm) = decode::u_type(instruction);
        self.auipc(rd, imm);
        false
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

    fn jal(&mut self, rd: u8, imm: u32) {
        self.write_reg(rd, self.pc.wrapping_add(4));
        self.pc = self.pc.wrapping_add(imm);
    }

    fn jalr(&mut self, rd: u8, rs1: u8, imm: u32) {
        self.write_reg(rd, self.pc.wrapping_add(4));
        self.pc = self.read_reg(rs1).wrapping_add(imm);
    }

    fn lui(&mut self, rd: u8, imm: u32) {
        self.write_reg(rd, imm << 12)
    }

    fn lw(&mut self, rd: u8, rs1: u8, imm: u32) {
        self.write_reg(rd, self.mem.load_word(self.read_reg(rs1).wrapping_add(imm)));
    }

    fn auipc(&mut self, rd: u8, imm: u32) {
        self.write_reg(rd, self.pc.wrapping_add(imm << 12))
    }

    fn srai(&mut self, rd: u8, rs1: u8, imm: u32) {
        self.write_reg(rd, ((self.read_reg(rs1) as i32) >> (imm & 0x1F)) as u32)
    }

    fn lb(&mut self, rd: u8, rs1: u8, imm: u32) {
        self.write_reg(
            rd,
            self.mem.load_byte(self.read_reg(rs1).wrapping_add(imm)) as i8 as i32 as u32,
        );
    }

    fn lbu(&mut self, rd: u8, rs1: u8, imm: u32) {
        self.write_reg(
            rd,
            self.mem.load_byte(self.read_reg(rs1).wrapping_add(imm)) as u32,
        )
    }

    fn lh(&mut self, rd: u8, rs1: u8, imm: u32) {
        self.write_reg(
            rd,
            self.mem.load_byte(self.read_reg(rs1).wrapping_add(imm)) as i16 as i32 as u32,
        );
    }

    fn lhu(&mut self, rd: u8, rs1: u8, imm: u32) {
        self.write_reg(
            rd,
            self.mem.load_half(self.read_reg(rs1).wrapping_add(imm)) as u32,
        )
    }

    fn sb(&mut self, rs1: u8, rs2: u8, imm: u32) {
        self.mem.store_byte(
            self.read_reg(rs1).wrapping_add(imm),
            self.read_reg(rs2) as u8,
        );
    }

    fn sh(&mut self, rs1: u8, rs2: u8, imm: u32) {
        self.mem.store_half(
            self.read_reg(rs1).wrapping_add(imm),
            self.read_reg(rs2) as u16,
        )
    }

    fn sw(&mut self, rs1: u8, rs2: u8, imm: u32) {
        self.mem
            .store_word(self.read_reg(rs1).wrapping_add(imm), self.read_reg(rs2))
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

    fn sltiu(&mut self, rd: u8, rs1: u8, imm: u32) {
        self.write_reg(rd, if self.read_reg(rs1) < imm { 1 } else { 0 });
    }

    fn beq(&mut self, rs1: u8, rs2: u8, imm: u32) -> bool {
        if self.read_reg(rs1) == self.read_reg(rs2) {
            self.pc = self.pc.wrapping_add(imm);
            true
        } else {
            false
        }
    }

    fn bne(&mut self, rs1: u8, rs2: u8, imm: u32) -> bool {
        if self.read_reg(rs1) != self.read_reg(rs2) {
            self.pc = self.pc.wrapping_add(imm);
            true
        } else {
            false
        }
    }

    fn blt(&mut self, rs1: u8, rs2: u8, imm: u32) -> bool {
        if (self.read_reg(rs1) as i32) < (self.read_reg(rs2) as i32) {
            self.pc = self.pc.wrapping_add(imm);
            true
        } else {
            false
        }
    }

    fn bge(&mut self, rs1: u8, rs2: u8, imm: u32) -> bool {
        if (self.read_reg(rs1) as i32) >= (self.read_reg(rs2) as i32) {
            self.pc = self.pc.wrapping_add(imm);
            true
        } else {
            false
        }
    }

    fn bltu(&mut self, rs1: u8, rs2: u8, imm: u32) -> bool {
        if self.read_reg(rs1) < self.read_reg(rs2) {
            self.pc = self.pc.wrapping_add(imm);
            true
        } else {
            false
        }
    }

    fn bgeu(&mut self, rs1: u8, rs2: u8, imm: u32) -> bool {
        if self.read_reg(rs1) >= self.read_reg(rs2) {
            self.pc = self.pc.wrapping_add(imm);
            true
        } else {
            false
        }
    }

    fn write_reg(&mut self, reg: u8, val: u32) {
        if reg != 0 {
            self.regs[reg as usize] = val;
        }
    }

    fn slti(&mut self, rd: u8, rs1: u8, imm: u32) {
        self.write_reg(
            rd,
            if (self.read_reg(rs1) as i32) < (imm as i32) {
                1
            } else {
                0
            },
        )
    }

    fn srli(&mut self, rd: u8, rs1: u8, imm: u32) {
        self.write_reg(rd, self.read_reg(rs1) >> (imm & 0x1F));
    }

    fn slli(&mut self, rd: u8, rs1: u8, imm: u32) {
        self.write_reg(rd, self.read_reg(rs1) << (imm & 0x1F));
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

    fn ori(&mut self, rd: u8, rs1: u8, imm: u32) {
        self.write_reg(rd, self.read_reg(rs1) | imm);
    }

    fn xor(&mut self, rd: u8, rs1: u8, rs2: u8) -> () {
        self.write_reg(rd, self.read_reg(rs1) ^ self.read_reg(rs2));
    }
    fn sub(&mut self, rd: u8, rs1: u8, rs2: u8) {
        self.write_reg(rd, self.read_reg(rs1).wrapping_sub(self.read_reg(rs2)));
    }

    fn xori(&mut self, rd: u8, rs1: u8, imm: u32) {
        self.write_reg(rd, self.read_reg(rs1) ^ imm);
    }

    fn add(&mut self, rd: u8, rs1: u8, rs2: u8) {
        self.write_reg(rd, self.read_reg(rs1).wrapping_add(self.read_reg(rs2)));
    }

    fn andi(&mut self, rd: u8, rs1: u8, imm: u32) {
        self.write_reg(rd, self.read_reg(rs1) & imm);
    }

    fn addi(&mut self, rd: u8, rs1: u8, imm: u32) {
        self.write_reg(rd, self.read_reg(rs1).wrapping_add(imm));
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
