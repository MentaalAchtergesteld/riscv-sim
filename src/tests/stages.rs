use crate::{components::*, instruction_formats::*, stages::*};

#[test]
fn test_instruction_fetch_and_pc_increment() {
    let mut pc = ProgramCounter::default();
    let mut memory = Memory::new(64);

    memory.write_word(0, 0x00000013).unwrap();
    memory.write_word(4, 0x00100093).unwrap();

    let instr1 = fetch_instruction(&pc, &memory).unwrap();
    assert_eq!(instr1, 0x00000013);

    pc.increment();

    let instr2 = fetch_instruction(&pc, &memory).unwrap();
    assert_eq!(instr2, 0x00100093);
}

#[test]
fn test_decode_addi() {
    let instruction = 0x00108093;

    let decoded_instruction = decode_instruction(instruction).expect("Couldn't decode instruction");

    let expected = DecodedInstr::I(IType {
        opcode: 0x13,
        rd: 1,
        func3: 0x0,
        rs1: 1,
        imm: 1,
        shamt: 1,
        func7: 0
    });

    assert_eq!(decoded_instruction, expected);
}

#[test]
fn test_execute_add() {
    let instruction = DecodedInstr::R(RType {
        opcode: 0b0110011,
        func: 0,
        func7: 0x00,
        func3: 0x0,
        rd: 1,
        rs1: 0,
        rs2: 0,
    });

    let execute_result = execute(&instruction, 1, 2, 0).unwrap();

    let writeback = execute_result.write_back.unwrap();
    assert_eq!(writeback.rd, 1);
    assert_eq!(writeback.value, 3);
}

#[test]
fn test_execute_sub() {
    let instruction = DecodedInstr::R(RType {
        opcode: 0b0110011,
        func: 0,
        func7: 0x20,
        func3: 0x0,
        rd: 1,
        rs1: 0,
        rs2: 0,
    });

    let execute_result = execute(&instruction, 5, 3, 0).unwrap();

    let writeback = execute_result.write_back.unwrap();
    assert_eq!(writeback.rd, 1);
    assert_eq!(writeback.value, 2);
}

#[test]
fn test_execute_sll() {
    let instruction = DecodedInstr::R(RType {
        opcode: 0b0110011,
        func: 0,
        func7: 0x00,
        func3: 0x1,
        rd: 1,
        rs1: 0,
        rs2: 0,
    });

    let execute_result = execute(&instruction, 32, 2, 0).unwrap();


    let writeback = execute_result.write_back.unwrap();
    assert_eq!(writeback.rd, 1);
    assert_eq!(writeback.value, 128);
}

#[test]
fn test_execute_slt() {
    let instruction = DecodedInstr::R(RType {
        opcode: 0b0110011,
        func: 0,
        func7: 0x00,
        func3: 0x2,
        rd: 1,
        rs1: 0,
        rs2: 0,
    });

    let execute_result = execute(&instruction, 5, -4, 0).unwrap();

    let writeback = execute_result.write_back.unwrap();
    assert_eq!(writeback.rd, 1);
    assert_eq!(writeback.value, 0);

    let execute_result = execute(&instruction, -1, 7, 0).unwrap();

    let writeback = execute_result.write_back.unwrap();
    assert_eq!(writeback.rd, 1);
    assert_eq!(writeback.value, 1);
}

#[test]
fn test_execute_sltu() {
    let instruction = DecodedInstr::R(RType {
        opcode: 0b0110011,
        func: 0,
        func7: 0x00,
        func3: 0x3,
        rd: 1,
        rs1: 0,
        rs2: 0,
    });

    let execute_result = execute(&instruction, 5, 4, 0).unwrap();

    let writeback = execute_result.write_back.unwrap();
    assert_eq!(writeback.rd, 1);
    assert_eq!(writeback.value, 0);

    let execute_result = execute(&instruction, 2, 10, 0).unwrap();

    let writeback = execute_result.write_back.unwrap();
    assert_eq!(writeback.rd, 1);
    assert_eq!(writeback.value, 1);
}

#[test]
fn test_execute_xor() {
    let instruction = DecodedInstr::R(RType {
        opcode: 0b0110011,
        func: 0,
        func7: 0x00,
        func3: 0x4,
        rd: 1,
        rs1: 0,
        rs2: 0,
    });

    let execute_result = execute(&instruction, 0b1111, 0b1010, 0).unwrap();

    let writeback = execute_result.write_back.unwrap();
    assert_eq!(writeback.rd, 1);
    assert_eq!(writeback.value, 0b0101);
}

#[test]
fn test_execute_srl() {
    let instruction = DecodedInstr::R(RType {
        opcode: 0b0110011,
        func: 0,
        func7: 0x00,
        func3: 0x5,
        rd: 1,
        rs1: 0,
        rs2: 0,
    });

    let execute_result = execute(&instruction, 128, 2, 0).unwrap();

    let writeback = execute_result.write_back.unwrap();
    assert_eq!(writeback.rd, 1);
    assert_eq!(writeback.value, 32);
}

#[test]
fn test_execute_sra() {
    let instruction = DecodedInstr::R(RType {
        opcode: 0b0110011,
        func: 0,
        func7: 0x20,
        func3: 0x5,
        rd: 1,
        rs1: 0,
        rs2: 0,
    });

    let execute_result = execute(&instruction, -128, 2, 0).unwrap();

    let writeback = execute_result.write_back.unwrap();
    assert_eq!(writeback.rd, 1);
    assert_eq!(writeback.value as i32, -32);
}

#[test]
fn test_execute_or() {
    let instruction = DecodedInstr::R(RType {
        opcode: 0b0110011,
        func: 0,
        func7: 0x00,
        func3: 0x6,
        rd: 1,
        rs1: 0,
        rs2: 0,
    });

    let execute_result = execute(&instruction, 0b1010, 0b0101, 0).unwrap();

    let writeback = execute_result.write_back.unwrap();
    assert_eq!(writeback.rd, 1);
    assert_eq!(writeback.value, 0b1111);
}

#[test]
fn test_execute_and() {
    let instruction = DecodedInstr::R(RType {
        opcode: 0b0110011,
        func: 0,
        func7: 0x00,
        func3: 0x7,
        rd: 1,
        rs1: 0,
        rs2: 0,
    });

    let execute_result = execute(&instruction, 0b1111, 0b1010, 0).unwrap();

    let writeback = execute_result.write_back.unwrap();
    assert_eq!(writeback.rd, 1);
    assert_eq!(writeback.value as i32, 0b1010);
}

#[test]
fn test_execute_jarl() {
    let instruction = DecodedInstr::I(IType {
        opcode: 0b1100111,
        func3: 0,
        rd: 1,
        rs1: 0,
        imm: 16,
        func7: 0,
        shamt: 0
    });

    let execute_result = execute(&instruction, 32, 0, 4).unwrap();

    let writeback = execute_result.write_back.unwrap();
    assert_eq!(writeback.rd, 1);
    assert_eq!(writeback.value, 8);

    let branch = execute_result.branch_addr.unwrap();
    assert_eq!(branch, 48);
}

#[test]
fn test_execute_lb() {
    let instruction = DecodedInstr::I(IType {
        opcode: 0b0000011,
        func3: 0x0,
        rd: 1,
        rs1: 0,
        imm: 4,
        func7: 0,
        shamt: 0
    });

    let execute_result = execute(&instruction, 32, 0, 4).unwrap();

    let read_mem = execute_result.read_mem.unwrap();
    assert_eq!(read_mem.rd, 1);
    assert_eq!(read_mem.address, 36);
    assert_eq!(read_mem.size, MemSize::Byte);
    assert_eq!(read_mem.signed, true);
}

#[test]
fn test_execute_lh() {
    let instruction = DecodedInstr::I(IType {
        opcode: 0b0000011,
        func3: 0x1,
        rd: 1,
        rs1: 0,
        imm: 4,
        func7: 0,
        shamt: 0
    });

    let execute_result = execute(&instruction, 32, 0, 4).unwrap();

    let read_mem = execute_result.read_mem.unwrap();
    assert_eq!(read_mem.rd, 1);
    assert_eq!(read_mem.address, 36);
    assert_eq!(read_mem.size, MemSize::Half);
    assert_eq!(read_mem.signed, true);
}

#[test]
fn test_execute_lw() {
    let instruction = DecodedInstr::I(IType {
        opcode: 0b0000011,
        func3: 0x2,
        rd: 1,
        rs1: 0,
        imm: 4,
        func7: 0,
        shamt: 0
    });

    let execute_result = execute(&instruction, 32, 0, 4).unwrap();

    let read_mem = execute_result.read_mem.unwrap();
    assert_eq!(read_mem.rd, 1);
    assert_eq!(read_mem.address, 36);
    assert_eq!(read_mem.size, MemSize::Word);
    assert_eq!(read_mem.signed, true);
}

#[test]
fn test_execute_ld() {
    let instruction = DecodedInstr::I(IType {
        opcode: 0b0000011,
        func3: 0x3,
        rd: 1,
        rs1: 0,
        imm: 4,
        func7: 0,
        shamt: 0
    });

    let execute_result = execute(&instruction, 32, 0, 4).unwrap();

    let read_mem = execute_result.read_mem.unwrap();
    assert_eq!(read_mem.rd, 1);
    assert_eq!(read_mem.address, 36);
    assert_eq!(read_mem.size, MemSize::Double);
    assert_eq!(read_mem.signed, true);
}

#[test]
fn test_execute_lbu() {
    let instruction = DecodedInstr::I(IType {
        opcode: 0b0000011,
        func3: 0x4,
        rd: 1,
        rs1: 0,
        imm: 4,
        func7: 0,
        shamt: 0
    });

    let execute_result = execute(&instruction, 32, 0, 4).unwrap();

    let read_mem = execute_result.read_mem.unwrap();
    assert_eq!(read_mem.rd, 1);
    assert_eq!(read_mem.address, 36);
    assert_eq!(read_mem.size, MemSize::Byte);
    assert_eq!(read_mem.signed, false);
}

#[test]
fn test_execute_lhu() {
    let instruction = DecodedInstr::I(IType {
        opcode: 0b0000011,
        func3: 0x5,
        rd: 1,
        rs1: 0,
        imm: 4,
        func7: 0,
        shamt: 0
    });

    let execute_result = execute(&instruction, 32, 0, 4).unwrap();

    let read_mem = execute_result.read_mem.unwrap();
    assert_eq!(read_mem.rd, 1);
    assert_eq!(read_mem.address, 36);
    assert_eq!(read_mem.size, MemSize::Half);
    assert_eq!(read_mem.signed, false);
}

#[test]
fn test_execute_addi() {
    let instruction = DecodedInstr::I(IType {
        opcode: 0b0010011,
        func3: 0x0,
        rd: 1,
        rs1: 0,
        imm: 4,
        func7: 0,
        shamt: 0
    });

    let execute_result = execute(&instruction, 32, 0, 4).unwrap();

    let writeback = execute_result.write_back.unwrap();
    assert_eq!(writeback.rd, 1);
    assert_eq!(writeback.value, 36);
}

#[test]
fn test_execute_slti() {
    let instruction = DecodedInstr::I(IType {
        opcode: 0b0010011,
        func3: 0x2,
        rd: 1,
        rs1: 0,
        imm: 4,
        func7: 0,
        shamt: 0
    });

    let execute_result = execute(&instruction, -1, 0, 4).unwrap();

    let writeback = execute_result.write_back.unwrap();
    assert_eq!(writeback.rd, 1);
    assert_eq!(writeback.value, 1);

    let execute_result = execute(&instruction, 32, 0, 4).unwrap();

    let writeback = execute_result.write_back.unwrap();
    assert_eq!(writeback.rd, 1);
    assert_eq!(writeback.value, 0);
}

#[test]
fn test_execute_sltiu() {
    let instruction = DecodedInstr::I(IType {
        opcode: 0b0010011,
        func3: 0x3,
        rd: 1,
        rs1: 0,
        imm: 4,
        func7: 0,
        shamt: 0
    });

    let execute_result = execute(&instruction, 1, 0, 4).unwrap();

    let writeback = execute_result.write_back.unwrap();
    assert_eq!(writeback.rd, 1);
    assert_eq!(writeback.value, 1);

    let execute_result = execute(&instruction, 32, 0, 4).unwrap();

    let writeback = execute_result.write_back.unwrap();
    assert_eq!(writeback.rd, 1);
    assert_eq!(writeback.value, 0);
}

#[test]
fn test_execute_xori() {
    let instruction = DecodedInstr::I(IType {
        opcode: 0b0010011,
        func3: 0x4,
        rd: 1,
        rs1: 0,
        imm: 0b1010,
        func7: 0,
        shamt: 0
    });

    let execute_result = execute(&instruction, 0b1111, 0, 4).unwrap();

    let writeback = execute_result.write_back.unwrap();
    assert_eq!(writeback.rd, 1);
    assert_eq!(writeback.value, 0b0101);
}

#[test]
fn test_execute_ori() {
    let instruction = DecodedInstr::I(IType {
        opcode: 0b0010011,
        func3: 0x6,
        rd: 1,
        rs1: 0,
        imm: 0b1010,
        func7: 0,
        shamt: 0
    });

    let execute_result = execute(&instruction, 0b0101, 0, 4).unwrap();

    let writeback = execute_result.write_back.unwrap();
    assert_eq!(writeback.rd, 1);
    assert_eq!(writeback.value, 0b1111);
}

#[test]
fn test_execute_andi() {
    let instruction = DecodedInstr::I(IType {
        opcode: 0b0010011,
        func3: 0x7,
        rd: 1,
        rs1: 0,
        imm: 0b1010,
        func7: 0,
        shamt: 0
    });

    let execute_result = execute(&instruction, 0b1111, 0, 4).unwrap();

    let writeback = execute_result.write_back.unwrap();
    assert_eq!(writeback.rd, 1);
    assert_eq!(writeback.value, 0b1010);
}

#[test]
fn test_execute_slli() {
    let instruction = DecodedInstr::I(IType {
        opcode: 0b0010011,
        func3: 0x1,
        rd: 1,
        rs1: 0,
        imm: 0,
        func7: 0,
        shamt: 2
    });

    let execute_result = execute(&instruction, 32, 0, 4).unwrap();

    let writeback = execute_result.write_back.unwrap();
    assert_eq!(writeback.rd, 1);
    assert_eq!(writeback.value, 128);
}

#[test]
fn test_execute_srli() {
    let instruction = DecodedInstr::I(IType {
        opcode: 0b0010011,
        func3: 0x5,
        rd: 1,
        rs1: 0,
        imm: 0,
        func7: 0x00,
        shamt: 2
    });

    let execute_result = execute(&instruction, 128, 0, 4).unwrap();

    let writeback = execute_result.write_back.unwrap();
    assert_eq!(writeback.rd, 1);
    assert_eq!(writeback.value, 32);
}

#[test]
fn test_execute_srai() {
    let instruction = DecodedInstr::I(IType {
        opcode: 0b0010011,
        func3: 0x5,
        rd: 1,
        rs1: 0,
        imm: 0,
        func7: 0x20,
        shamt: 2
    });

    let execute_result = execute(&instruction, -128, 0, 4).unwrap();

    let writeback = execute_result.write_back.unwrap();
    assert_eq!(writeback.rd, 1);
    assert_eq!(writeback.value as i32, -32);
}

#[test]
fn test_execute_fence() {}

#[test]
fn test_execute_fencei() {}

#[test]
fn test_execute_ecall() {}

#[test]
fn test_execute_ebreak() {}

#[test]
fn test_execute_csrrw() {}

#[test]
fn test_execute_csrrs() {}

#[test]
fn test_execute_csrrc() {}

#[test]
fn test_execute_csrrwi() {}

#[test]
fn test_execute_csrrsi() {}

#[test]
fn test_execute_csrrci() {}

#[test]
fn test_execute_sb() {
    let instruction = DecodedInstr::S(SType {
        opcode: 0b0100011,
        func: 0x0,
        rs1: 0,
        rs2: 0,
        imm: 16,
    });

    let execute_result = execute(&instruction, 32, 0xCF, 4).unwrap();

    let write_mem = execute_result.write_mem.unwrap();
    assert_eq!(write_mem.address, 48);
    assert_eq!(write_mem.data, 0xCF);
    assert_eq!(write_mem.size, MemSize::Byte);
}

#[test]
fn test_execute_sh() {
    let instruction = DecodedInstr::S(SType {
        opcode: 0b0100011,
        func: 0x1,
        rs1: 0,
        rs2: 0,
        imm: 16,
    });

    let execute_result = execute(&instruction, 32, 0xCFCF, 4).unwrap();

    let write_mem = execute_result.write_mem.unwrap();
    assert_eq!(write_mem.address, 48);
    assert_eq!(write_mem.data, 0xCFCF);
    assert_eq!(write_mem.size, MemSize::Half);
}

#[test]
fn test_execute_sw() {
    let instruction = DecodedInstr::S(SType {
        opcode: 0b0100011,
        func: 0x2,
        rs1: 0,
        rs2: 0,
        imm: 16,
    });

    let execute_result = execute(&instruction, 32, 0x1FCFCFCF, 4).unwrap();

    let write_mem = execute_result.write_mem.unwrap();
    assert_eq!(write_mem.address, 48);
    assert_eq!(write_mem.data, 0x1FCFCFCF);
    assert_eq!(write_mem.size, MemSize::Word);
}

#[test]
fn test_execute_sd() {
    let instruction = DecodedInstr::S(SType {
        opcode: 0b0100011,
        func: 0x3,
        rs1: 0,
        rs2: 0,
        imm: 16,
    });

    let execute_result = execute(&instruction, 32, 0x1FCFCFCFFFCFCFCF, 4).unwrap();

    let write_mem = execute_result.write_mem.unwrap();
    assert_eq!(write_mem.address, 48);
    assert_eq!(write_mem.data, 0x1FCFCFCFFFCFCFCF);
    assert_eq!(write_mem.size, MemSize::Double);
}

#[test]
fn test_execute_beq() {
    let instruction = DecodedInstr::B(BType {
        opcode: 0b1100011,
        func: 0x0,
        rs1: 0,
        rs2: 0,
        imm: 16,
    });

    let execute_result = execute(&instruction, 32, 16, 4).unwrap();
    assert_eq!(execute_result.branch_addr, None);

    let execute_result = execute(&instruction, 32, 32, 4).unwrap();
    assert_eq!(execute_result.branch_addr.unwrap(), 20);
}

#[test]
fn test_execute_bne() {
    let instruction = DecodedInstr::B(BType {
        opcode: 0b1100011,
        func: 0x1,
        rs1: 0,
        rs2: 0,
        imm: 16,
    });

    let execute_result = execute(&instruction, 32, 32, 4).unwrap();
    assert_eq!(execute_result.branch_addr, None);

    let execute_result = execute(&instruction, 32, 16, 4).unwrap();
    assert_eq!(execute_result.branch_addr.unwrap(), 20);
}

#[test]
fn test_execute_blt() {
    let instruction = DecodedInstr::B(BType {
        opcode: 0b1100011,
        func: 0x4,
        rs1: 0,
        rs2: 0,
        imm: 16,
    });

    let execute_result = execute(&instruction, -32, -32, 4).unwrap();
    assert_eq!(execute_result.branch_addr, None);

    let execute_result = execute(&instruction, -16, 32, 4).unwrap();
    assert_eq!(execute_result.branch_addr.unwrap(), 20);
}

#[test]
fn test_execute_bge() {
    let instruction = DecodedInstr::B(BType {
        opcode: 0b1100011,
        func: 0x5,
        rs1: 0,
        rs2: 0,
        imm: 16,
    });

    let execute_result = execute(&instruction, -16, 32, 4).unwrap();
    assert_eq!(execute_result.branch_addr, None);

    let execute_result = execute(&instruction, -32, -32, 4).unwrap();
    assert_eq!(execute_result.branch_addr.unwrap(), 20);

    let execute_result = execute(&instruction, 32, -16, 4).unwrap();
    assert_eq!(execute_result.branch_addr.unwrap(), 20);
}

#[test]
fn test_execute_bltu() {
    let instruction = DecodedInstr::B(BType {
        opcode: 0b1100011,
        func: 0x6,
        rs1: 0,
        rs2: 0,
        imm: 16,
    });

    let execute_result = execute(&instruction, 32, 32, 4).unwrap();
    assert_eq!(execute_result.branch_addr, None);

    let execute_result = execute(&instruction, 16, 32, 4).unwrap();
    assert_eq!(execute_result.branch_addr.unwrap(), 20);
}

#[test]
fn test_execute_bgeu() {
    let instruction = DecodedInstr::B(BType {
        opcode: 0b1100011,
        func: 0x7,
        rs1: 0,
        rs2: 0,
        imm: 16,
    });

    let execute_result = execute(&instruction, 16, 32, 4).unwrap();
    assert_eq!(execute_result.branch_addr, None);

    let execute_result = execute(&instruction, 32, 32, 4).unwrap();
    assert_eq!(execute_result.branch_addr.unwrap(), 20);

    let execute_result = execute(&instruction, 32, 16, 4).unwrap();
    assert_eq!(execute_result.branch_addr.unwrap(), 20);
}

#[test]
fn test_execute_lui() {
    let instruction = DecodedInstr::U(UType {
        opcode: 0b0110111,
        rd: 1,
        imm: 0x12345000,
    });

    let execute_result = execute(&instruction, 0, 0, 4).unwrap();
    
    let writeback = execute_result.write_back.unwrap();
    assert_eq!(writeback.rd, 1);
    assert_eq!(writeback.value, 0x12345000);
}

#[test]
fn test_execute_auipc() {
    let instruction = DecodedInstr::U(UType {
        opcode: 0b0010111,
        rd: 1,
        imm: 0x12345000,
    });

    let execute_result = execute(&instruction, 0, 0, 4).unwrap();
    
    let writeback = execute_result.write_back.unwrap();
    assert_eq!(writeback.rd, 1);
    assert_eq!(writeback.value, 0x12345004);
}

#[test]
fn test_execute_jal() {
    let instruction = DecodedInstr::J(JType {
        opcode: 0b1101111,
        rd: 1,
        imm: 32,
    });

    let execute_result = execute(&instruction, 0, 0, 4).unwrap();
    
    let writeback = execute_result.write_back.unwrap();
    assert_eq!(writeback.rd, 1);
    assert_eq!(writeback.value, 8);

    assert_eq!(execute_result.branch_addr.unwrap(), 36);
}