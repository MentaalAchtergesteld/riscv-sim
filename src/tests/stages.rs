use crate::{components::{Memory, ProgramCounter}, decoder::DecodedInstr, instruction_formats::IType, stages::{decode_instruction, instruction_fetch}};

#[test]
fn test_instruction_fetch_and_pc_increment() {
    let mut pc = ProgramCounter::default();
    let mut memory = Memory::new(64);

    memory.write_word(0, 0x00000013).unwrap();
    memory.write_word(4, 0x00100093).unwrap();

    let instr1 = instruction_fetch(&pc, &memory).unwrap();
    assert_eq!(instr1, 0x00000013);

    pc.increment();

    let instr2 = instruction_fetch(&pc, &memory).unwrap();
    assert_eq!(instr2, 0x00100093);
}

#[test]
fn test_decode_addi() {
    let pc = ProgramCounter { address: 0x1000 };
    let instruction = 0x00108093;

    let result = decode_instruction(instruction, pc.address).expect("Couldn't decode instruction");

    assert_eq!(result.pc, 0x1000);

    let expected = DecodedInstr::I(IType {
        opcode: 0x13,
        rd: 1,
        func: 0x0,
        rs1: 1,
        imm: 1
    });

    assert_eq!(result.instruction, expected);
}