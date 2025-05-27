use crate::instruction_formats::*;
use crate::util::*;

#[derive(Debug, PartialEq)]
pub enum DecodedInstr {
    R(RType),
    I(IType),
    S(SType),
    B(BType),
    U(UType),
    J(JType)
}

pub fn decode(instruction: u32) -> Option<DecodedInstr> {
    let opcode = extract_bits(instruction, 6, 0) as u8;

    match opcode {
        0b0110011 => Some(DecodedInstr::R(RType::from(instruction))),
        0b0010011 => Some(DecodedInstr::I(IType::from(instruction))),
        0b0000011 => Some(DecodedInstr::I(IType::from(instruction))),
        0b0100011 => Some(DecodedInstr::S(SType::from(instruction))),
        0b1100011 => Some(DecodedInstr::B(BType::from(instruction))),
        0b1101111 => Some(DecodedInstr::J(JType::from(instruction))),
        0b1100111 => Some(DecodedInstr::I(IType::from(instruction))),
        0b0110111 => Some(DecodedInstr::U(UType::from(instruction))),
        0b0010111 => Some(DecodedInstr::U(UType::from(instruction))),
        0b1110011 => Some(DecodedInstr::I(IType::from(instruction))),
        0b0011011 => Some(DecodedInstr::I(IType::from(instruction))),
        0b0111011 => Some(DecodedInstr::R(RType::from(instruction))),
        0b0001111 => Some(DecodedInstr::I(IType::from(instruction))),
        _ => None
    }
}