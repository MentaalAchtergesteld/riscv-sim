use crate::stages::{decode_instruction, execute, fetch_instruction, DecodeError, DecodedInstr, ExecuteError, MemSize};

#[derive(Default)]
pub struct ProgramCounter {
    pub address: u64
}

impl ProgramCounter {
    pub fn increment(&mut self) {
        self.address += 4;
    }

    pub fn set(&mut self, value: u64) {
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

    pub fn read_byte(&self, addr: usize, signed: bool) -> Result<u64, MemoryError> {
        if addr >= self.data.len() { return Err(MemoryError::OutOfBounds { address: addr, max: self.data.len() }) }
        let byte = self.data[addr];

        if signed {
            Ok((byte as i8) as i32 as u64)
        } else {
            Ok(byte as u64)
        }
    }

    pub fn read_half_word(&self, addr: usize, signed: bool) -> Result<u64, MemoryError> {
        if addr+1 >= self.data.len() { return Err(MemoryError::OutOfBounds { address: addr, max: self.data.len() }) }
        let bytes = &self.data[addr..addr+2];
        let half = u16::from_le_bytes([bytes[0], bytes[1]]);

        if signed {
            Ok((half as i16) as i32 as u64)
        } else {
            Ok(half as u64)
        }
    }

    pub fn read_word(&self, addr: usize) -> Result<u64, MemoryError> {
        if addr+3 >= self.data.len() { return Err(MemoryError::OutOfBounds { address: addr, max: self.data.len() }) }
        let bytes = &self.data[addr..addr+4];
        Ok(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]) as u64)
    }

    pub fn read_double_word(&self, addr: usize) -> Result<u64, MemoryError> {
        if addr+7 >= self.data.len() { return Err(MemoryError::OutOfBounds { address: addr, max: self.data.len() }) }
        let bytes = &self.data[addr..addr+8];
        Ok(u64::from_le_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3],
            bytes[4], bytes[5], bytes[6], bytes[7]
        ]))
    }

    pub fn write_byte(&mut self, addr: usize, val: u64) -> Result<(), MemoryError> {
        if addr >= self.data.len() { return Err(MemoryError::OutOfBounds { address: addr, max: self.data.len() }) }
        self.data[addr] = val as u8;
        Ok(())
    }

    pub fn write_half_word(&mut self, addr: usize, val: u64) -> Result<(), MemoryError> {
        if addr+1 >= self.data.len() { return Err(MemoryError::OutOfBounds { address: addr, max: self.data.len() }) }
        let bytes = (val as u16).to_le_bytes();
        self.data[addr..addr+2].copy_from_slice(&bytes);
        Ok(())
    }

    pub fn write_word(&mut self, addr: usize, val: u64) -> Result<(), MemoryError> {
        if addr+3 >= self.data.len() { return Err(MemoryError::OutOfBounds { address: addr, max: self.data.len() }) }
        let bytes = (val as u32).to_le_bytes();
        self.data[addr..addr+4].copy_from_slice(&bytes);
        Ok(())
    }

    pub fn write_double_word(&mut self, addr: usize, val: u64) -> Result<(), MemoryError> {
        if addr+7 >= self.data.len() { return Err(MemoryError::OutOfBounds { address: addr, max: self.data.len() }) }
        let bytes = val.to_le_bytes();
        self.data[addr..addr+8].copy_from_slice(&bytes);
        Ok(())
    }
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
    ElfParseError,
    #[error("Too little memory to load ELF file")]
    ElfTooLittleMemoryError,

    #[error("Fetch error at PC={pc}: {source}")]
    FetchError {
        source: MemoryError,
        pc: u64
    },

    #[error("Memory error a PC={pc}: {source}")]
    MemoryError {
        source: MemoryError,
        pc: u64
    },

    #[error("Decode error at PC={pc}: {source}")]
    DecodeError {
        source: DecodeError,
        pc: u64,
    },

    #[error("Execute error at PC={pc}: {source}")]
    ExecuteError {
        source: ExecuteError,
        pc: u64
    },
}

pub struct CPU {
    pub pc: ProgramCounter,
    pub mem: Memory,
    pub regs: [u64; 32],

    pub last_store: Option<(u64, u64)>,
}

impl CPU {
    pub fn new(mem_size: usize) -> Self {
        CPU {
            pc: ProgramCounter::default(),
            mem: Memory::new(mem_size),
            regs: [0; 32],
            last_store: None,
        }
    }

    pub fn cycle(&mut self) -> Result<(), CPUError> {
        self.last_store = None;
        let instruction = fetch_instruction(&self.pc, &self.mem)
            .map_err(|e| CPUError::FetchError { source: e, pc: self.pc.address })?;
        let decoded_instruction = decode_instruction(instruction as u32)
            .map_err(|e| CPUError::DecodeError { source: e, pc: self.pc.address })?;

        let rs1_val = match &decoded_instruction {
            DecodedInstr::R(r) => self.regs[r.rs1 as usize],
            DecodedInstr::I(i) => self.regs[i.rs1 as usize],
            DecodedInstr::S(s) => self.regs[s.rs1 as usize],
            DecodedInstr::B(b) => self.regs[b.rs1 as usize],
            DecodedInstr::U(_) => 0,
            DecodedInstr::J(_) => 0,
        } as i64;

        let rs2_val = match &decoded_instruction {
            DecodedInstr::R(r) => self.regs[r.rs2 as usize],
            DecodedInstr::I(_) => 0,
            DecodedInstr::S(s) => self.regs[s.rs2 as usize],
            DecodedInstr::B(b) => self.regs[b.rs2 as usize],
            DecodedInstr::U(_) => 0,
            DecodedInstr::J(_) => 0,
        } as i64;

        let execute_result = execute(&decoded_instruction, rs1_val, rs2_val, self.pc.address)
            .map_err(|e| CPUError::ExecuteError { source: e, pc: self.pc.address })?;

        if let Some(read_mem) = execute_result.read_mem {
            let data = match read_mem.size {
                MemSize::Byte => self.mem.read_byte(read_mem.address as usize, read_mem.signed),
                MemSize::Half => self.mem.read_half_word(read_mem.address as usize, read_mem.signed),
                MemSize::Word => self.mem.read_word(read_mem.address as usize),
                MemSize::Double => self.mem.read_double_word(read_mem.address as usize),
            }.map_err(|e| CPUError::MemoryError { source: e, pc: self.pc.address })?;

            if read_mem.rd != 0 {
                self.regs[read_mem.rd as usize] = data;
            }
        }

        if let Some(write_mem) = execute_result.write_mem {
            match write_mem.size {
                MemSize::Byte => self.mem.write_byte(write_mem.address as usize, write_mem.data),
                MemSize::Half => self.mem.write_half_word(write_mem.address as usize, write_mem.data),
                MemSize::Word => self.mem.write_word(write_mem.address as usize, write_mem.data),
                MemSize::Double => self.mem.write_double_word(write_mem.address as usize, write_mem.data),
            }.map_err(|e| CPUError::MemoryError { source: e, pc: self.pc.address })?;

            self.last_store = Some((write_mem.address, write_mem.data));
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

// Beautiful code written by Chat, because i couldn't be bothered to write this shit myself.

use goblin::elf::{program_header::PT_LOAD, Elf};

impl CPU {
    pub fn load_elf(&mut self, elf_bytes: &[u8]) -> Result<(), CPUError> {
        let elf = Elf::parse(elf_bytes).map_err(|_| CPUError::ElfParseError)?;

        // Laad elk PT_LOAD segment
        for ph in &elf.program_headers {
            if ph.p_type != PT_LOAD {
                continue;
            }

            let vaddr = ph.p_vaddr;
            let offset = ph.p_offset as usize;
            let file_size = ph.p_filesz as usize;
            let mem_size  = ph.p_memsz as usize;
            // let mem_off = (vaddr - base) as usize;
            let mem_off = vaddr as usize;

            // Zorg dat we binnen geheugenruimte blijven
            if mem_off + mem_size > self.mem.data.len() {
                return Err(CPUError::ElfTooLittleMemoryError);
            }

            // Kopieer initieel bestand
            self.mem.data[mem_off .. mem_off + file_size]
                .copy_from_slice(&elf_bytes[offset .. offset + file_size]);

            // Zero-fill voor resterende bytes (bijv. BSS)
            for b in &mut self.mem.data[mem_off + file_size .. mem_off + mem_size] {
                *b = 0;
            }
        }

        self.pc.set(elf.entry as u64);

        Ok(())
    }
}