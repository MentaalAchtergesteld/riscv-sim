mod tests;

fn extract_bits(instruction: u32, high: u8, low: u8) -> u32 {
    let width = high - low + 1;
    (instruction >> low) & ((1u32 << width) - 1)
}

pub struct RTYpe {
    opcode: u8,
    rd: u8,
    funct3: u8,
    rs1: u8,
    rs2: u8,
    funct7: u8
}

impl From<u32> for RTYpe {
    fn from(value: u32) -> Self {
        let opcode = extract_bits(value, 6, 0)   as u8;
        let rd = extract_bits(value, 11, 7)      as u8;
        let funct3 = extract_bits(value, 14, 12) as u8;
        let rs1 = extract_bits(value, 19, 15)    as u8;
        let rs2 = extract_bits(value, 24, 20)    as u8;
        let funct7 = extract_bits(value, 31, 25) as u8;

        Self {
            opcode,
            rd,
            funct3,
            rs1,
            rs2,
            funct7
        }
    }
}

pub struct IType {
    opcode: u8,
    rd: u8,
    funct3: u8,
    rs1: u8,
    imm: i32
}

impl From<u32> for IType {
    fn from(value: u32) -> Self {
        let opcode = extract_bits(value, 6, 0)   as u8;
        let rd = extract_bits(value, 11, 7)      as u8;
        let funct3 = extract_bits(value, 14, 12) as u8;
        let rs1 = extract_bits(value, 19, 15)    as u8;
        let imm = extract_bits(value, 31, 20)   as i32;

        Self {
            opcode,
            rd,
            funct3,
            rs1,
            imm
        }
    }
}

pub struct SType {
    opcode: u8,
    imm: i32,
    funct3: u8,
    rs1: u8,
    rs2: u8,
}

impl From<u32> for SType {
    fn from(value: u32) -> Self {
        let opcode = extract_bits(value, 6, 0)   as u8;
        let imm1 = extract_bits(value, 11, 7)    as u8;
        let funct3 = extract_bits(value, 14, 12) as u8;
        let rs1 = extract_bits(value, 19, 15)    as u8;
        let rs2 = extract_bits(value, 24, 20)    as u8;
        let imm2 = extract_bits(value, 31, 25)   as u8;

        let imm_raw = ((imm2 as u32) << 5) | (imm1 as u32);
        let imm = ((imm_raw << 20) as i32) >> 20;

        Self {
            opcode,
            imm,
            funct3,
            rs1,
            rs2
        }
    }
}

struct BType {
    opcode: u8,
    imm: i32,
    funct3: u8,
    rs1: u8,
    rs2: u8,
}

struct UType {
    opcode: u8,
    rd: u8,
    imm: i32
}

struct JType {
    opcode: u8,
    rd: u8,
    imm: i32
}

fn main() {
    println!("Hello, world!");
}
