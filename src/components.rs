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

#[derive(Debug)]
pub enum MemoryError {
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

    pub fn read_byte(&self, addr: usize) -> Result<u8, MemoryError> {
        if addr >= self.data.len() { return Err(MemoryError::OutOfBounds) }
        Ok(self.data[addr])
    }

    pub fn read_half_word(&self, addr: usize) -> Result<u16, MemoryError> {
        if addr+2 >= self.data.len() { return Err(MemoryError::OutOfBounds) }
        let bytes = &self.data[addr..addr+2];
        Ok(u16::from_le_bytes([bytes[0], bytes[1]]))
    }

    pub fn read_word(&self, addr: usize) -> Result<u32, MemoryError> {
        if addr+4 >= self.data.len() { return Err(MemoryError::OutOfBounds) }
        let bytes = &self.data[addr..addr+4];
        Ok(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
    }

    pub fn read_double_word(&self, addr: usize) -> Result<u64, MemoryError> {
        if addr+8 >= self.data.len() { return Err(MemoryError::OutOfBounds) }
        let bytes = &self.data[addr..addr+8];
        Ok(u64::from_le_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3],
            bytes[4], bytes[5], bytes[6], bytes[7]
        ]))
    }

    pub fn write_byte(&mut self, addr: usize, val: u8) -> Result<(), MemoryError> {
        if addr >= self.data.len() { return Err(MemoryError::OutOfBounds) }
        self.data[addr] = val;
        Ok(())
    }

    pub fn write_half_word(&mut self, addr: usize, val: u16) -> Result<(), MemoryError> {
        if addr+2 >= self.data.len() { return Err(MemoryError::OutOfBounds) }
        let bytes = val.to_le_bytes();
        self.data[addr..addr+2].copy_from_slice(&bytes);
        Ok(())
    }

    pub fn write_word(&mut self, addr: usize, val: u32) -> Result<(), MemoryError> {
        if addr+4 >= self.data.len() { return Err(MemoryError::OutOfBounds) }
        let bytes = val.to_le_bytes();
        self.data[addr..addr+4].copy_from_slice(&bytes);
        Ok(())
    }

    pub fn write_double_word(&mut self, addr: usize, val: u64) -> Result<(), MemoryError> {
        if addr+8 >= self.data.len() { return Err(MemoryError::OutOfBounds) }
        let bytes = val.to_le_bytes();
        self.data[addr..addr+8].copy_from_slice(&bytes);
        Ok(())
    }
}