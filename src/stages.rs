use crate::{components::{Memory, MemoryError, ProgramCounter}, instruction_formats::{BType, IType, JType, RType, SType, UType}, util::extract_bits};

pub fn fetch_instruction(pc: &ProgramCounter, memory: &Memory) -> Result<u32, MemoryError> {
    memory.read_word(pc.address as usize)
}

#[derive(Debug, PartialEq, Clone)]
pub enum DecodedInstr {
    R(RType),
    I(IType),
    S(SType),
    B(BType),
    U(UType),
    J(JType)
}

#[derive(Debug, thiserror::Error)]
pub enum DecodeError {
    #[error("Unknown opcode: {0:8x}")]
    UnknownOpcode(u8),
    #[error("End of program")]
    EndOfProgram,
}

pub fn decode_instruction(instruction: u32) -> Result<DecodedInstr, DecodeError> {
    let opcode = extract_bits(instruction, 6, 0) as u8;

    match opcode {
        0b0110011 => Ok(DecodedInstr::R(RType::from(instruction))),
        0b0010011 => Ok(DecodedInstr::I(IType::from(instruction))),
        0b0000011 => Ok(DecodedInstr::I(IType::from(instruction))),
        0b0100011 => Ok(DecodedInstr::S(SType::from(instruction))),
        0b1100011 => Ok(DecodedInstr::B(BType::from(instruction))),
        0b1101111 => Ok(DecodedInstr::J(JType::from(instruction))),
        0b1100111 => Ok(DecodedInstr::I(IType::from(instruction))),
        0b0110111 => Ok(DecodedInstr::U(UType::from(instruction))),
        0b0010111 => Ok(DecodedInstr::U(UType::from(instruction))),
        0b1110011 => Ok(DecodedInstr::I(IType::from(instruction))),
        0b0011011 => Ok(DecodedInstr::I(IType::from(instruction))),
        0b0111011 => Ok(DecodedInstr::R(RType::from(instruction))),
        0b0001111 => Ok(DecodedInstr::I(IType::from(instruction))),
        0b1111111 => Err(DecodeError::EndOfProgram),
        _ => Err(DecodeError::UnknownOpcode(opcode))
    }
}

#[derive(Debug, PartialEq)]
pub enum MemSize {
    Byte,
    Half,
    Word,
    // DoubleWord
}

#[derive(Debug, PartialEq)]
pub struct WriteMem {
    pub address: u32,
    pub data: u32,
    pub size: MemSize
}

#[derive(Debug, PartialEq)]
pub struct ReadMem {
    pub address: u32,
    pub size: MemSize,
    pub rd: u8,
    pub signed: bool,
}

#[derive(Debug, PartialEq)]
pub struct WriteBack {
    pub rd: u8,
    pub value: u32
}

#[derive(Default)]
pub struct ExecuteResult {
    // pub alu_result: Option<u32>,
    pub read_mem: Option<ReadMem>,
    pub write_mem: Option<WriteMem>,
    pub write_back: Option<WriteBack>,
    pub branch_addr: Option<u32>,
}

impl ExecuteResult {
    pub fn with_read_mem(mut self, read_mem: ReadMem) -> Self {
        self.read_mem = Some(read_mem);
        self
    }

    pub fn with_write_mem(mut self, write_mem: WriteMem) -> Self {
        self.write_mem = Some(write_mem);
        self
    }

    pub fn with_write_back(mut self, write_back: WriteBack) -> Self {
        self.write_back = Some(write_back);
        self
    }

    pub fn with_branch(mut self, branch: u32) -> Self {
        self.branch_addr = Some(branch);
        self
    }
}

pub fn execute_r(r: &RType, rs1_val: i32, rs2_val: i32) -> Option<ExecuteResult> {
    match r.opcode {
        0b0110011 => match (r.func7, r.func3) {
            (0x00, 0x0) => { // ADD Add
                Some(ExecuteResult::default()
                    .with_write_back(WriteBack { rd: r.rd, value: (rs1_val.wrapping_add(rs2_val)) as u32 })
                )
            },
            (0x20, 0x0) => { // SUB Subtract
                Some(ExecuteResult::default()
                    .with_write_back(WriteBack { rd: r.rd, value: (rs1_val.wrapping_sub(rs2_val)) as u32})
                )
            },
            (0x00, 0x1) => { // SLL Shift Left Logical
                Some(ExecuteResult::default()
                    .with_write_back(WriteBack { rd: r.rd, value: (rs1_val << (rs2_val & 0x1F)) as u32})
                )
            },
            (0x00, 0x2) => { // SLT Set less than
                Some(ExecuteResult::default()
                    .with_write_back(WriteBack { rd: r.rd, value: (rs1_val < rs2_val) as u32})
                )
            },
            (0x00, 0x3) => { // SLTU Set less than unsigned
                Some(ExecuteResult::default()
                    .with_write_back(WriteBack { rd: r.rd, value: ((rs1_val as u32) < (rs2_val as u32)) as u32})
                )
            },
            (0x00, 0x4) => { // XOR
                Some(ExecuteResult::default()
                    .with_write_back(WriteBack { rd: r.rd, value: (rs1_val ^ rs2_val) as u32})
                )
            },
            (0x00, 0x5) => { // SRL Shift right logical
                Some(ExecuteResult::default()
                    .with_write_back(WriteBack { rd: r.rd, value: ((rs1_val as u32) >> (rs2_val & 0x1f)) as u32})
                )
            },
            (0x20, 0x5) => { // SRA Shift right arithmetic
                Some(ExecuteResult::default()
                    .with_write_back(WriteBack { rd: r.rd, value: ((rs1_val as i32) >> (rs2_val & 0x1f)) as u32})
                )
            },
            (0x00, 0x6) => { // OR
                Some(ExecuteResult::default()
                    .with_write_back(WriteBack { rd: r.rd, value: (rs1_val | rs2_val) as u32})
                )
            },
            (0x00, 0x7) => { // AND
                Some(ExecuteResult::default()
                    .with_write_back(WriteBack { rd: r.rd, value: (rs1_val & rs2_val) as u32})
                )
            }
            _ => None
        }
        _ => None
    }
}

pub fn execute_i(i: &IType, rs1_val: i32, pc: u32) -> Option<ExecuteResult> {
    match i.opcode {
        0b1100111 => {// JARL
            Some(ExecuteResult::default()
                .with_write_back(WriteBack { rd: i.rd, value: pc.wrapping_add(4) })
                .with_branch((rs1_val.wrapping_add(i.imm) & !1) as u32)
            )
        },
        0b0000011 => match i.func3 {
            0x0 => { // LB Load byte
                Some(ExecuteResult::default()
                    .with_read_mem(ReadMem { address: rs1_val.wrapping_add(i.imm) as u32, size: MemSize::Byte, rd: i.rd, signed: true })
                )
            },
            0x1 => { // LH Load half word
                Some(ExecuteResult::default()
                    .with_read_mem(ReadMem { address: rs1_val.wrapping_add(i.imm) as u32, size: MemSize::Half, rd: i.rd, signed: true })
                )
            },
            0x2 => { // LW Load word
                Some(ExecuteResult::default()
                    .with_read_mem(ReadMem { address: rs1_val.wrapping_add(i.imm) as u32, size: MemSize::Word, rd: i.rd, signed: true })
                )
            },
            0x3 => { // LBU Load byte unsigned
                Some(ExecuteResult::default()
                    .with_read_mem(ReadMem { address: rs1_val.wrapping_add(i.imm) as u32, size: MemSize::Byte, rd: i.rd, signed: false })
                )
            },
            0x4 => { // LHU Load half word unsigned
                Some(ExecuteResult::default()
                    .with_read_mem(ReadMem { address: rs1_val.wrapping_add(i.imm) as u32, size: MemSize::Half, rd: i.rd, signed: false })
                )
            },
            _ => None
        },
        0b0010011 => match i.func3 {
            0x0 => { // ADDI Add immediate
                Some(ExecuteResult::default()
                    .with_write_back(WriteBack { rd: i.rd, value: (rs1_val.wrapping_add(i.imm)) as u32 })
                )
            },
            0x2 => { // SLTI Set less than immediate
                Some(ExecuteResult::default()
                    .with_write_back(WriteBack { rd: i.rd, value: (rs1_val < i.imm) as u32 })
                )
            },
            0x3 => { // SLTIU Set less than immediate unsigned
                Some(ExecuteResult::default()
                    .with_write_back(WriteBack { rd: i.rd, value: ((rs1_val as u32) < (i.imm as u32)) as u32 })
                )
            },
            0x4 => { // XORI XOR immediate
                Some(ExecuteResult::default()
                    .with_write_back(WriteBack { rd: i.rd, value: (rs1_val ^ i.imm) as u32 })
                )
            },
            0x6 => { // ORI OR immediate
                Some(ExecuteResult::default()
                    .with_write_back(WriteBack { rd: i.rd, value: (rs1_val | i.imm) as u32 })
                )
            },
            0x7 => { // ANDI AND immediate
                Some(ExecuteResult::default()
                    .with_write_back(WriteBack { rd: i.rd, value: (rs1_val & i.imm) as u32 })
                )
            },
            0x1 => { // SLLI Shift left logical immediate
                Some(ExecuteResult::default()
                    .with_write_back(WriteBack { rd: i.rd, value: (rs1_val << (i.imm & 0x1f)) as u32 })
                )
            },
            0x5 => { 
                match i.func7 {
                    0x0 => Some(ExecuteResult::default() // SRLI Shift right logical immediate
                        .with_write_back(WriteBack { rd: i.rd, value: ((rs1_val as u32) >> (i.imm & 0x1f)) as u32 })
                    ),
                    0x1 => Some(ExecuteResult::default() // SRAI Shift right arithmetic immediate
                        .with_write_back(WriteBack { rd: i.rd, value: ((rs1_val as i32) >> (i.imm & 0x1f)) as u32})
                    ),
                    _ => None
                }
            }
            _ => None
        },
        0b0001111 => {
            match i.func3 {
                0x0 => { // FENCE Wait until all memory operations finished
                    None
                },
                0x1 => { // FENCE.I Synchronize the instruction cache with memoryr
                    None
                },
                _ => None,
            }
        },
        0b1110011 => {
            match i.func3 {
                0x0 => match i.imm {
                    0x0 => { // ECALL Environment Call
                        None
                    },
                    0x1 => { // EBREAK Environment Breakpoint
                        None
                    },
                    _ => None
                },
                0x1 => { // CSRRW CSR Read/Write
                    None
                },
                0x2 => { // CSRRS CSR Read/Set
                    None
                },
                0x3 => { // CSRRC CSR Read/Clear
                    None
                },
                0x5 => { // CSRRWI CSR Read/Write Immediate
                    None
                },
                0x6 => { // CSRRSI CSR Read/Set Immediate
                    None
                },
                0x7 => { // CSRRCI CSR Read/Clear Immediate
                    None
                }
                _ => None
            }
        }
        _ => None
    }
}

pub fn execute_s(s: &SType, rs1_val: i32, rs2_val: i32) -> Option<ExecuteResult> {
    match s.opcode {
        0b0100011 => match s.func {
            0x0 => { // SB Store byte
                Some(ExecuteResult::default()
                    .with_write_mem(WriteMem { address: rs1_val.wrapping_add(s.imm) as u32, data: (rs2_val & 0xFF) as u32, size: MemSize::Byte })
                )
            },
            0x1 => { // SH Store half word
                Some(ExecuteResult::default()
                    .with_write_mem(WriteMem { address: rs1_val.wrapping_add(s.imm) as u32, data: (rs2_val & 0xFFFF) as u32, size: MemSize::Half })
                )
            },
            0x2 => { // SW Store word
                Some(ExecuteResult::default()
                    .with_write_mem(WriteMem { address: rs1_val.wrapping_add(s.imm) as u32, data: rs2_val as u32, size: MemSize::Word })
                )
            }
            _ => None
        }
        _ => None
    }
}

pub fn execute_b(b: &BType, rs1_val: i32, rs2_val: i32, pc: u32) -> Option<ExecuteResult> {
    match b.opcode {
        0b1100011 => match b.func {
            0x0 => { // BEQ Branch if equal
                if rs1_val == rs2_val {
                    Some(ExecuteResult::default()
                        .with_branch(pc.wrapping_add(b.imm as u32))
                    )
                } else {
                    Some(ExecuteResult::default())
                }
            },
            0x1 => { // BNE Branch if not equal
                if rs1_val != rs2_val {
                    Some(ExecuteResult::default()
                        .with_branch(pc.wrapping_add(b.imm as u32))
                    )
                } else {
                    Some(ExecuteResult::default())
                }
            },
            0x4 => { // BLT Branch if lesser than
                if rs1_val < rs2_val {
                    Some(ExecuteResult::default()
                        .with_branch(pc.wrapping_add(b.imm as u32))
                    )
                } else {
                    Some(ExecuteResult::default())
                }
            },
            0x5 => { // BGE Branch if greater than or equal
                if rs1_val >= rs2_val {
                    Some(ExecuteResult::default()
                        .with_branch(pc.wrapping_add(b.imm as u32))
                    )
                } else {
                    Some(ExecuteResult::default())
                }
            },
            0x6 => { // BLTU Branch if lesser than (unsigned)
                if (rs1_val as u32) < (rs2_val as u32) {
                    Some(ExecuteResult::default()
                        .with_branch(pc.wrapping_add(b.imm as u32))
                    )
                } else {
                    Some(ExecuteResult::default())
                }
            },
            0x7 => { // BGEU Branch if greater than or equal (unsigned)
                if (rs1_val as u32) >= (rs2_val as u32) {
                    Some(ExecuteResult::default()
                        .with_branch(pc.wrapping_add(b.imm as u32))
                    )
                } else {
                    Some(ExecuteResult::default())
                }
            },
            _ => None
        },
        _ => None
    }
}

pub fn execute_u(u: &UType, pc: u32) -> Option<ExecuteResult> {
    match u.opcode {
        0b0110111 => { // LUI Load upper immediate
            Some(ExecuteResult::default()
                .with_write_back(WriteBack { rd: u.rd, value: u.imm as u32 })
            )
        },
        0b0010111 => { // AUIPC Add upper immediate to PC
            Some(ExecuteResult::default()
                .with_write_back(WriteBack { rd: u.rd, value: pc.wrapping_add(u.imm as u32) })
            )
        },
        _ => None
    }
}

pub fn execute_j(j: &JType, pc: u32) -> Option<ExecuteResult> {
    match j.opcode {
        0b1101111 => { //JAL Jump and link
            Some(ExecuteResult::default()
                .with_write_back(WriteBack { rd: j.rd, value: pc + 4 })
                .with_branch(pc.wrapping_add(j.imm as u32))
            )
        }
        _ => None
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ExecuteError {
    #[error("Unimplemented instruction: {instruction:?}, type: {instr_type}")]
    UnimplementedInstruction{ instr_type: String , instruction: DecodedInstr },
}

pub fn execute(instruction: &DecodedInstr, rs1_val: i32, rs2_val: i32, pc: u32) -> Result<ExecuteResult, ExecuteError> {
    match instruction {
        DecodedInstr::R(r) => execute_r(r, rs1_val, rs2_val)
            .ok_or(ExecuteError::UnimplementedInstruction { instr_type: "R".into(), instruction: instruction.clone() }),
        DecodedInstr::I(i) => execute_i(i, rs1_val, pc)
            .ok_or(ExecuteError::UnimplementedInstruction { instr_type: "I".into(), instruction: instruction.clone() }),
        DecodedInstr::S(s) => execute_s(s, rs1_val, rs2_val)
            .ok_or(ExecuteError::UnimplementedInstruction { instr_type: "S".into(), instruction: instruction.clone() }),
        DecodedInstr::B(b) => execute_b(b, rs1_val, rs2_val, pc)
            .ok_or(ExecuteError::UnimplementedInstruction { instr_type: "B".into(), instruction: instruction.clone() }),
        DecodedInstr::U(u) => execute_u(u, pc)
            .ok_or(ExecuteError::UnimplementedInstruction { instr_type: "U".into(), instruction: instruction.clone() }),
        DecodedInstr::J(j) => execute_j(j, pc)
            .ok_or(ExecuteError::UnimplementedInstruction { instr_type: "J".into(), instruction: instruction.clone() }),
    }
}