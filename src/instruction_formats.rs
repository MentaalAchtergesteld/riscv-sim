use crate::util::extract_bits;

#[derive(Debug, PartialEq, Clone)]
pub struct RType {
    pub opcode: u8,
    pub rd: u8,
    pub func3: u8,
    pub rs1: u8,
    pub rs2: u8,
    pub func7: u8,
    pub func: u16,
}

impl From<u32> for RType {
    fn from(value: u32) -> Self {
        let opcode = extract_bits(value, 6, 0)    as u8;
        let rd = extract_bits(value, 11, 7)       as u8;
        let func3 = extract_bits(value, 14, 12)   as u8;
        let rs1 = extract_bits(value, 19, 15)     as u8;
        let rs2 = extract_bits(value, 24, 20)     as u8;
        let func7 = extract_bits(value, 31, 25)   as u8;

        let func = ((func7 as u16) << 3) | (func3 as u16);

        Self {
            opcode,
            rd,
            rs1,
            rs2,
            func,
            func3,
            func7
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct IType {
    pub opcode: u8,
    pub rd: u8,
    pub func3: u8,
    pub rs1: u8,
    pub shamt: u8,
    pub func7: u8,
    pub imm: i32,
}

impl From<u32> for IType {
    fn from(value: u32) -> Self {
        let opcode = extract_bits(value, 6, 0) as u8;
        let rd = extract_bits(value, 11, 7)    as u8;
        let func3 = extract_bits(value, 14, 12) as u8;
        let rs1 = extract_bits(value, 19, 15)  as u8;
        let imm_raw = extract_bits(value, 31, 20);

        let shamt = (imm_raw & 0x1F) as u8;
        let func7 = (imm_raw >> 5) as u8;

        let imm = ((imm_raw as i32) << 20) >> 20;

        Self {
            opcode,
            rd,
            func3,
            rs1,
            imm,
            shamt,
            func7
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct SType {
    pub opcode: u8,
    pub imm: i32,
    pub func: u16,
    pub rs1: u8,
    pub rs2: u8,
}

impl From<u32> for SType {
    fn from(value: u32) -> Self {
        let opcode = extract_bits(value, 6, 0)      as u8;
        let imm_4_0 = extract_bits(value, 11, 7);
        let func = extract_bits(value, 14, 12)     as u16;
        let rs1 = extract_bits(value, 19, 15)       as u8;
        let rs2 = extract_bits(value, 24, 20)       as u8;
        let imm_11_5 = extract_bits(value, 31, 25);

        let imm_raw = (imm_11_5 << 5) | imm_4_0;
        let imm = ((imm_raw << 20) as i32) >> 20;

        Self {
            opcode,
            imm,
            func,
            rs1,
            rs2
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct BType {
    pub opcode: u8,
    pub imm: i32,
    pub func: u16,
    pub rs1: u8,
    pub rs2: u8,
}

impl From<u32> for BType {
    fn from(value: u32) -> Self {
        let opcode = extract_bits(value, 6, 0)      as u8;
        let imm_11 = extract_bits(value, 7, 7);
        let imm_4_1 = extract_bits(value, 11, 8);
        let func = extract_bits(value, 14, 12)      as u16;
        let rs1 = extract_bits(value, 19, 15)       as u8;
        let rs2 = extract_bits(value, 24, 20)       as u8;
        let imm_10_5 = extract_bits(value, 30, 25);
        let imm_12 = extract_bits(value, 31, 31);

        let imm_raw = (imm_12   << 12) |
                           (imm_11   << 11) |
                           (imm_10_5 << 5 ) |
                           (imm_4_1  << 1 );
        
        let imm = ((imm_raw as i32) << 19) >> 19;

        Self {
            opcode,
            imm,
            func,
            rs1,
            rs2
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct UType {
    pub opcode: u8,
    pub rd: u8,
    pub imm: i32
}

impl From<u32> for UType {
    fn from(value: u32) -> Self {
        let opcode = extract_bits(value, 6, 0)     as u8;
        let rd = extract_bits(value, 11, 7)        as u8;
        let imm_raw = extract_bits(value, 31, 12);

        let imm = (imm_raw << 12) as i32;

        println!("U type instruction: 0b{:032b}", value);
        println!("U type imm_raw: 0x{:08x}", imm_raw);
        println!("U type imm: 0x{:08x}", imm);

        Self {
            opcode,
            rd,
            imm
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct JType {
    pub opcode: u8,
    pub rd: u8,
    pub imm: i32
}

impl From<u32> for JType {
    fn from(value: u32) -> Self {
        let opcode = extract_bits(value, 6, 0)       as u8;
        let rd = extract_bits(value, 11, 7)          as u8;
        let imm_19_12 = extract_bits(value, 19, 12);
        let imm_11 = extract_bits(value, 20, 20);
        let imm_10_1 = extract_bits(value, 30, 21);
        let imm_20 = extract_bits(value, 31, 31);

        let imm_raw = (imm_20 << 20)    |
                           (imm_19_12 << 12) |
                           (imm_11 << 11)    |
                           (imm_10_1 << 1);
        
        let imm = ((imm_raw as i32) << 11) >> 11;

        Self {
            opcode,
            rd,
            imm
        }
    }
}

