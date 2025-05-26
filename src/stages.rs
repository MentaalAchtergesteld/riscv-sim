use crate::{components::{Memory, MemoryError, ProgramCounter}, decoder::{decode, DecodedInstr}};

pub fn instruction_fetch(pc: &ProgramCounter, memory: &Memory) -> Result<u32, MemoryError> {
    memory.read_word(pc.address as usize)
}

pub struct InstructionDecodeResult {
    pub instruction: DecodedInstr,
    pub pc: u32,
}

pub fn decode_instruction(instruction: u32, pc_address: u32) -> Option<InstructionDecodeResult> {
    Some(InstructionDecodeResult {
        instruction: decode(instruction)?,
        pc: pc_address
    })
}

pub enum AluResult {
    Value(i32),
    Branch { taken: bool, target: u32 }
}

pub fn execute(instruction: &DecodedInstr, rs1_val: i32, rs2_val: i32, pc: u32) -> AluResult {
    match instruction {
        DecodedInstr::R(r) => match r.func {
            0x0 => AluResult::Value(rs1_val + rs2_val),
            0x1 => AluResult::Value(rs1_val << (rs2_val & 0x1F)),

            _ => panic!("Unimplemented R-Type func: {}", r.func)
        },
        DecodedInstr::I(i) => match i.opcode {
            0b0000011 => AluResult::Value(0),
            0b0010011 => match i.func {
                0x0 => AluResult::Value(rs1_val + i.imm),
                _ => panic!("Unimplemented I-Type func: {}, opcode: {}", i.func, i.opcode)
            },
            _ => panic!("Unimplemented I-Type opcode: {}", i.opcode)
        }
        DecodedInstr::S(s) => match s.func {
            _ => panic!("Unimplemented S-Type func: {}", s.func)
        },
        DecodedInstr::B(b) => {
            let taken = match b.func {
                0x0 => rs1_val == rs2_val,
                0x1 => rs1_val != rs2_val,
                _ => false,
            };

            let target = pc.wrapping_add(b.imm as u32);
            AluResult::Branch { taken, target }
        },
        DecodedInstr::U(u) => match u.opcode {
            0b0110111 => AluResult::Value(u.imm),
            0b0010111 => AluResult::Value(pc as i32 + u.imm),
            _ => panic!("Unimplemented U-Type opcode: {}", u.opcode)
        },
        DecodedInstr::J(j) =>  match j.opcode {
            0b1101111 => AluResult::Branch { taken: true, target: pc.wrapping_add(j.imm as u32) },
            _ => panic!("Unimplemented J-Type opcode: {}", j.opcode)
        }
    }
}