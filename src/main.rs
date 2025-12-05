use crate::cpu::CPU;

mod cpu;
mod decode;
mod memory;

fn main() {}

fn run(instructions: &[u8]) {
    let mut cpu = CPU::default();
    // todo don't just crash due to oob lol
    loop {
        let c_instruction = u16::from_le_bytes([
            instructions[cpu.pc as usize],
            instructions[cpu.pc as usize + 1],
        ]);
        if decode::is_compressed_instr(c_instruction) {
            let jumped = cpu.interpret_c(c_instruction);
            if jumped {
                cpu.pc = cpu.pc.wrapping_add(2);
            }
        } else {
            let instruction = u32::from_le_bytes(
                instructions[cpu.pc as usize..cpu.pc as usize + 4]
                    .try_into()
                    .unwrap(),
            );
            let jumped = cpu.interpret(instruction);
            if jumped {
                cpu.pc = cpu.pc.wrapping_add(4);
            }
        }
    }
}
