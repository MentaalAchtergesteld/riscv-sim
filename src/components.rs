use crate::stages::{decode_instruction, execute, fetch_instruction, DecodeError, DecodedInstr, ExecuteError, MemSize};

#[derive(Default)]
pub struct ProgramCounter {
    pub address: u32
}

impl ProgramCounter {
    pub fn increment(&mut self) {
        self.address += 4;
    }

    pub fn set(&mut self, value: u32) {
        self.address = value;
    }
}

#[derive(Debug, thiserror::Error)]
pub enum MemoryError {
    #[error("Memory address out of bounds")]
    OutOfBounds
}

#[derive(Default)]
pub struct Memory {
    pub data: Vec<u8>
}

impl Memory {
    pub fn new(size: usize) -> Self {
        Self {
            data: vec![0; size],
        }
    }

    pub fn read_byte(&self, addr: usize, signed: bool) -> Result<u32, MemoryError> {
        if addr >= self.data.len() { return Err(MemoryError::OutOfBounds) }
        let byte = self.data[addr];

        if signed {
            Ok((byte as i8) as i32 as u32)
        } else {
            Ok(byte as u32)
        }
    }

    pub fn read_half_word(&self, addr: usize, signed: bool) -> Result<u32, MemoryError> {
        if addr+1 >= self.data.len() { return Err(MemoryError::OutOfBounds) }
        let bytes = &self.data[addr..addr+2];
        let half = u16::from_le_bytes([bytes[0], bytes[1]]);

        if signed {
            Ok((half as i16) as i32 as u32)
        } else {
            Ok(half as u32)
        }
    }

    pub fn read_word(&self, addr: usize) -> Result<u32, MemoryError> {
        if addr+3 >= self.data.len() { return Err(MemoryError::OutOfBounds) }
        let bytes = &self.data[addr..addr+4];
        Ok(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
    }

    // pub fn read_double_word(&self, addr: usize) -> Result<u64, MemoryError> {
    //     if addr+7 >= self.data.len() { return Err(MemoryError::OutOfBounds) }
    //     let bytes = &self.data[addr..addr+8];
    //     Ok(u64::from_le_bytes([
    //         bytes[0], bytes[1], bytes[2], bytes[3],
    //         bytes[4], bytes[5], bytes[6], bytes[7]
    //     ]))
    // }

    pub fn write_byte(&mut self, addr: usize, val: u32) -> Result<(), MemoryError> {
        if addr >= self.data.len() { return Err(MemoryError::OutOfBounds) }
        self.data[addr] = val as u8;
        Ok(())
    }

    pub fn write_half_word(&mut self, addr: usize, val: u32) -> Result<(), MemoryError> {
        if addr+1 >= self.data.len() { return Err(MemoryError::OutOfBounds) }
        let bytes = (val as u16).to_le_bytes();
        self.data[addr..addr+2].copy_from_slice(&bytes);
        Ok(())
    }

    pub fn write_word(&mut self, addr: usize, val: u32) -> Result<(), MemoryError> {
        if addr+3 >= self.data.len() { return Err(MemoryError::OutOfBounds) }
        let bytes = val.to_le_bytes();
        self.data[addr..addr+4].copy_from_slice(&bytes);
        Ok(())
    }

    // pub fn write_double_word(&mut self, addr: usize, val: u64) -> Result<(), MemoryError> {
    //     if addr+7 >= self.data.len() { return Err(MemoryError::OutOfBounds) }
    //     let bytes = val.to_le_bytes();
    //     self.data[addr..addr+8].copy_from_slice(&bytes);
    //     Ok(())
    // }
}

#[derive(thiserror::Error, Debug)]
pub enum CPUError {
    #[error("Fetch error: {0}")]
    FetchError(#[from] MemoryError),

    #[error("Decode error: {0}")]
    DecodeError(#[from] DecodeError),

    #[error("Execute error: {0}")]
    ExecuteError(#[from] ExecuteError),
}

pub struct CPU {
    pub pc: ProgramCounter,
    pub instr_mem: Memory,
    pub data_mem: Memory,
    pub regs: [u32; 32],
}

impl CPU {
    pub fn new(instr_mem_size: usize, data_mem_size: usize) -> Self {
        CPU {
            pc: ProgramCounter::default(),
            instr_mem: Memory::new(instr_mem_size),
            data_mem: Memory::new(data_mem_size),
            regs: [0; 32]
        }
    }

    pub fn cycle(&mut self) -> Result<(), CPUError> {
        let instruction = fetch_instruction(&self.pc, &self.instr_mem)?;
        let decoded_instruction = decode_instruction(instruction)?;

        let rs1_val = match &decoded_instruction {
            DecodedInstr::R(r) => self.regs[r.rs1 as usize],
            DecodedInstr::I(i) => self.regs[i.rs1 as usize],
            DecodedInstr::S(s) => self.regs[s.rs1 as usize],
            DecodedInstr::B(b) => self.regs[b.rs1 as usize],
            DecodedInstr::U(_) => 0,
            DecodedInstr::J(_) => 0,
        } as i32;

        let rs2_val = match &decoded_instruction {
            DecodedInstr::R(r) => self.regs[r.rs2 as usize],
            DecodedInstr::I(_) => 0,
            DecodedInstr::S(s) => self.regs[s.rs2 as usize],
            DecodedInstr::B(b) => self.regs[b.rs2 as usize],
            DecodedInstr::U(_) => 0,
            DecodedInstr::J(_) => 0,
        } as i32;

        let execute_result = execute(&decoded_instruction, rs1_val, rs2_val, self.pc.address)?;

        if let Some(read_mem) = execute_result.read_mem {
            let data = match read_mem.size {
                MemSize::Byte => self.data_mem.read_byte(read_mem.address as usize, read_mem.signed),
                MemSize::Half => self.data_mem.read_half_word(read_mem.address as usize, read_mem.signed),
                MemSize::Word => self.data_mem.read_word(read_mem.address as usize),
            }?;

            self.regs[read_mem.rd as usize] = data;
        }

        if let Some(write_mem) = execute_result.write_mem {
            match write_mem.size {
                MemSize::Byte => self.data_mem.write_byte(write_mem.address as usize, write_mem.data),
                MemSize::Half => self.data_mem.write_half_word(write_mem.address as usize, write_mem.data),
                MemSize::Word => self.data_mem.write_word(write_mem.address as usize, write_mem.data),
            }?;
        }

        if let Some(write_back) = execute_result.write_back {
            self.regs[write_back.rd as usize] = write_back.value;
        }

        if let Some(branch_addr) = execute_result.branch_addr {
            self.pc.set(branch_addr);
        } else {
            self.pc.increment();
        }

        Ok(())
    }
}