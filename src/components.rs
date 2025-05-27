use goblin::elf::{program_header::{PF_X, PT_LOAD},  Elf};

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
    #[error("Memory address out of bounds. Requested address: {address}, max: {max}")]
    OutOfBounds{address: usize, max: usize},
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
        if addr >= self.data.len() { return Err(MemoryError::OutOfBounds { address: addr, max: self.data.len() }) }
        let byte = self.data[addr];

        if signed {
            Ok((byte as i8) as i32 as u32)
        } else {
            Ok(byte as u32)
        }
    }

    pub fn read_half_word(&self, addr: usize, signed: bool) -> Result<u32, MemoryError> {
        if addr+1 >= self.data.len() { return Err(MemoryError::OutOfBounds { address: addr, max: self.data.len() }) }
        let bytes = &self.data[addr..addr+2];
        let half = u16::from_le_bytes([bytes[0], bytes[1]]);

        if signed {
            Ok((half as i16) as i32 as u32)
        } else {
            Ok(half as u32)
        }
    }

    pub fn read_word(&self, addr: usize) -> Result<u32, MemoryError> {
        if addr+3 >= self.data.len() { return Err(MemoryError::OutOfBounds { address: addr, max: self.data.len() }) }
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
        if addr >= self.data.len() { return Err(MemoryError::OutOfBounds { address: addr, max: self.data.len() }) }
        self.data[addr] = val as u8;
        Ok(())
    }

    pub fn write_half_word(&mut self, addr: usize, val: u32) -> Result<(), MemoryError> {
        if addr+1 >= self.data.len() { return Err(MemoryError::OutOfBounds { address: addr, max: self.data.len() }) }
        let bytes = (val as u16).to_le_bytes();
        self.data[addr..addr+2].copy_from_slice(&bytes);
        Ok(())
    }

    pub fn write_word(&mut self, addr: usize, val: u32) -> Result<(), MemoryError> {
        if addr+3 >= self.data.len() { return Err(MemoryError::OutOfBounds { address: addr, max: self.data.len() }) }
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

impl std::fmt::Display for Memory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("MEMORY | SIZE: {} \n", self.data.len()))?;

        for address in (0..self.data.len()).step_by(4) {
            f.write_str(&format!("0x{:08x}: 0x{:08x} | 0b{:032b}\n", address, self.read_word(address).unwrap(), self.read_word(address).unwrap()))?;
        }

        f.write_str("---")
    }
}

#[derive(thiserror::Error, Debug)]
pub enum CPUError {
    #[error("Couldn't load ELF binary")]
    ElfParsError,
    #[error("Too little memory to load ELF file")]
    ElfTooLittleMemoryError,
    #[error("No elf text section found")]
    ElfNoTextSection,

    #[error("Fetch error at PC={pc}: {source}")]
    FetchError {
        source: MemoryError,
        pc: u32
    },

    #[error("Memory error a PC={pc}: {source}")]
    MemoryError {
        source: MemoryError,
        pc: u32
    },

    #[error("Decode error at PC={pc}: {source}")]
    DecodeError {
        source: DecodeError,
        pc: u32,
    },

    #[error("Execute error at PC={pc}: {source}")]
    ExecuteError {
        source: ExecuteError,
        pc: u32
    },
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

    pub fn load_elf(&mut self, elf_bytes: &[u8]) -> Result<(), CPUError> {
        let elf = Elf::parse(elf_bytes).map_err(|_| CPUError::ElfParsError)?;

        let base = elf.program_headers
                    .iter()
                    .filter(|ph| ph.p_type == PT_LOAD && ph.p_flags & PF_X != 0)
                    .map(|ph| ph.p_vaddr)
                    .min()
                    .unwrap_or(0);

        // Laad alleen PT_LOAD segments met het X-bit
        for ph in &elf.program_headers {
            if ph.p_type == PT_LOAD && (ph.p_flags & PF_X) != 0 {
                let start = (ph.p_vaddr - base) as usize;
                let len   = ph.p_filesz as usize;  // NB: p_filesz, niet p_memsz
                let range = ph.file_range();       // offset..offset+p_filesz

                if start + len > self.instr_mem.data.len() {
                    return Err(CPUError::ElfTooLittleMemoryError);
                }
                self.instr_mem.data[start .. start + len]
                    .copy_from_slice(&elf_bytes[range]);
            }
        }

        // Zet PC op het entrypoint relativ to base
        self.pc.set((elf.entry - base) as u32);

        println!("start pc: x{:08x}", self.pc.address);
        Ok(())
    }


    pub fn cycle(&mut self) -> Result<(), CPUError> {
        let instruction = fetch_instruction(&self.pc, &self.instr_mem)
            .map_err(|e| CPUError::FetchError { source: e, pc: self.pc.address })?;
        let decoded_instruction = decode_instruction(instruction)
            .map_err(|e| CPUError::DecodeError { source: e, pc: self.pc.address })?;

        // println!("PC: {:08x}", self.pc.address);
        // println!("instr: {:032b}", instruction);
        println!("decoded instr: {:?}", decoded_instruction);

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

        let execute_result = execute(&decoded_instruction, rs1_val, rs2_val, self.pc.address)
            .map_err(|e| CPUError::ExecuteError { source: e, pc: self.pc.address })?;

        if let Some(read_mem) = execute_result.read_mem {
            let data = match read_mem.size {
                MemSize::Byte => self.data_mem.read_byte(read_mem.address as usize, read_mem.signed),
                MemSize::Half => self.data_mem.read_half_word(read_mem.address as usize, read_mem.signed),
                MemSize::Word => self.data_mem.read_word(read_mem.address as usize),
            }.map_err(|e| CPUError::MemoryError { source: e, pc: self.pc.address })?;

            if read_mem.rd != 0 {
                self.regs[read_mem.rd as usize] = data;
            }
        }

        if let Some(write_mem) = execute_result.write_mem {
            match write_mem.size {
                MemSize::Byte => self.data_mem.write_byte(write_mem.address as usize, write_mem.data),
                MemSize::Half => self.data_mem.write_half_word(write_mem.address as usize, write_mem.data),
                MemSize::Word => self.data_mem.write_word(write_mem.address as usize, write_mem.data),
            }.map_err(|e| CPUError::MemoryError { source: e, pc: self.pc.address })?;
        }

        if let Some(write_back) = execute_result.write_back {
            if write_back.rd != 0 {
                self.regs[write_back.rd as usize] = write_back.value;
            }
        }

        if let Some(branch_addr) = execute_result.branch_addr {
            self.pc.set(branch_addr);
        } else {
            self.pc.increment();
        }

        Ok(())
    }
}