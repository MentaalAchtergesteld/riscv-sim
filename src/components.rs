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

    pub fn read_double_word(&self, addr: usize) -> Result<u64, MemoryError> {
        if addr+7 >= self.data.len() { return Err(MemoryError::OutOfBounds) }
        let bytes = &self.data[addr..addr+8];
        Ok(u64::from_le_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3],
            bytes[4], bytes[5], bytes[6], bytes[7]
        ]))
    }

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

    pub fn write_double_word(&mut self, addr: usize, val: u64) -> Result<(), MemoryError> {
        if addr+7 >= self.data.len() { return Err(MemoryError::OutOfBounds) }
        let bytes = val.to_le_bytes();
        self.data[addr..addr+8].copy_from_slice(&bytes);
        Ok(())
    }
}