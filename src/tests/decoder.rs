use std::{collections::HashMap, fs, io, path::Path};
use crate::{instruction_formats::*, stages::{decode_instruction, DecodedInstr}};

#[test]
fn test_rtype_decode() {
    let raw = 0x41FB8633;
    let r = RType::from(raw);

    assert_eq!(r.opcode, 0x33);
    assert_eq!(r.rd, 0x0C);
    assert_eq!(r.rs1, 0x17);
    assert_eq!(r.rs2, 0x1F);
    assert_eq!(r.func, 0x100);
}

#[test]
fn test_itype_decode() {
    let raw = 0xFFCBE613;
    let i = IType::from(raw);

    assert_eq!(i.opcode, 0x13);
    assert_eq!(i.rd, 0x0C);
    assert_eq!(i.func3, 0x06);
    assert_eq!(i.rs1, 0x17);
    assert_eq!(i.imm, -0x04);
}

#[test]
fn test_stype_decode() {
    let raw = 0xFE752C23;
    let s = SType::from(raw);

    assert_eq!(s.opcode, 0x23);
    assert_eq!(s.func, 0x02);
    assert_eq!(s.rs1, 0xA);
    assert_eq!(s.rs2, 0x7);
    assert_eq!(s.imm, -0x008);
}

#[test]
fn test_btype_decode() {
    let raw = 0x80209163;
    let b = BType::from(raw);

    assert_eq!(b.opcode, 0x63);
    assert_eq!(b.func, 0x01);
    assert_eq!(b.rs1, 0x1);
    assert_eq!(b.rs2, 0x2);
    assert_eq!(b.imm, -0xFFE);
}

#[test]
fn test_utype_decode() {
    let raw = 0x123455B7;
    let u = UType::from(raw);

    assert_eq!(u.opcode, 0x37);
    assert_eq!(u.rd, 0x0B);
    assert_eq!(u.imm, 0x12345000);
}

#[test]
fn test_jtype_decode() {
    let raw = 0x802000EF;
    let j = JType::from(raw);

    assert_eq!(j.opcode, 0x6F);
    assert_eq!(j.rd, 0x01);
    assert_eq!(j.imm, -0xFFFFE);
}

#[test]
fn test_opcode_decode() {
    let instructions = [
        (0x33, DecodedInstr::R(RType::from(0))),
        (0x13, DecodedInstr::I(IType::from(0))),
        (0x03, DecodedInstr::I(IType::from(0))),
        (0x23, DecodedInstr::S(SType::from(0))),
        (0x63, DecodedInstr::B(BType::from(0))),
        (0x6F, DecodedInstr::J(JType::from(0))),
        (0x67, DecodedInstr::I(IType::from(0))),
        (0x37, DecodedInstr::U(UType::from(0))),
        (0x17, DecodedInstr::U(UType::from(0))),
        (0x73, DecodedInstr::I(IType::from(0)))
    ];

    fn same_variant(a: &DecodedInstr, b: &DecodedInstr) -> bool {
        std::mem::discriminant(a) == std::mem::discriminant(b)
    }

    for (instruction, expected) in instructions {
        let decoded = decode_instruction(instruction).unwrap();
        assert!(same_variant(&decoded, &expected));
    }
}

fn read_hex_instructions(test: &str) -> Result<Vec<u32>, io::Error> {
    let filename = format!("./testdata/instructions/{}-instrs.txt", test);
    let path = Path::new(&filename);
    let instruction_text = std::fs::read_to_string(path)?;

    let hex = instruction_text.lines()
        .filter_map(|line| {
            let trimmed = line.trim();
            if trimmed.is_empty() { return None; }

            u32::from_str_radix(&trimmed[2..], 16).ok()
        })
        .collect::<Vec<u32>>();

    Ok(hex)
}

fn read_key_pair(keypair: &str) -> Option<(&str, u32)> {
    if let Some((key, value_str)) = keypair.split_once("=") {
        let value = if let Some(hex) = value_str.strip_prefix("0x") {
            u32::from_str_radix(hex, 16).ok()?
        } else {
            value_str.parse::<u32>().ok()?
        };

        Some((key, value))
    } else {
        None
    }
}

fn read_expected_instructions(test: &str) -> Result<Vec<DecodedInstr>, io::Error> {
    let filename = format!("./testdata/instructions/{}.test", test);
    let path = Path::new(&filename);
    let instruction_text = fs::read_to_string(path)?;

    let expected = instruction_text.lines()
        .skip(1)
        .filter_map(|line| {
            let (_, data_str) = line.split_once("	")?;

            let mut splitted_data = data_str.split_whitespace().collect::<Vec<&str>>();

            let instr_type = splitted_data.remove(1);

            let data = splitted_data.iter()
                .filter_map(|keypair| read_key_pair(&keypair))
                .collect::<HashMap<&str, u32>>();

            let opcode = *data.get("op")? as u8;

            match instr_type {
                "R-type" => {
                    Some(DecodedInstr::R(RType {
                        opcode,
                        rd: *data.get("rd").unwrap_or(&0) as u8,
                        rs1: *data.get("rs1").unwrap_or(&0) as u8,
                        rs2: *data.get("rs2").unwrap_or(&0) as u8,
                        func: *data.get("func").unwrap_or(&0) as u16,
                        func3: (*data.get("func").unwrap_or(&0) & 0x7) as u8,
                        func7: (*data.get("func").unwrap_or(&0) >> 3) as u8,
                    }))
                },
                "I-type" => {
                    Some(DecodedInstr::I(IType {
                        opcode,
                        rd: *data.get("rd").unwrap_or(&0) as u8,
                        func3: *data.get("func").unwrap_or(&0) as u8,
                        rs1: *data.get("rs1").unwrap_or(&0) as u8,
                        imm: ((*data.get("imm").unwrap_or(&0) as i32) << 20) >> 20,
                        shamt: (*data.get("imm").unwrap_or(&0) & 0x1F) as u8,
                        func7: (*data.get("imm").unwrap_or(&0) >> 5) as u8,
                    }))
                },
                "S-type" => {
                    Some(DecodedInstr::S(SType {
                        opcode,
                        imm: ((*data.get("imm").unwrap_or(&0) as i32) << 20) >> 20,
                        func: *data.get("func").unwrap_or(&0) as u16,
                        rs1: *data.get("rs1").unwrap_or(&0) as u8,
                        rs2: *data.get("rs2").unwrap_or(&0) as u8
                    }))
                },
                "B-type" => {
                    Some(DecodedInstr::B(BType {
                        opcode,
                        imm: ((*data.get("imm").unwrap_or(&0) as i32) << 19) >> 19,
                        func: *data.get("func").unwrap_or(&0) as u16,
                        rs1: *data.get("rs1").unwrap_or(&0) as u8,
                        rs2: *data.get("rs2").unwrap_or(&0) as u8
                    }))
                },
                "U-type" => {
                    Some(DecodedInstr::U(UType {
                        opcode,
                        rd: *data.get("rd").unwrap_or(&0) as u8,
                        imm: *data.get("imm").unwrap_or(&0) as i32,
                    }))
                },
                "J-type" => {
                    Some(DecodedInstr::J(JType {
                        opcode,
                        rd: *data.get("rd").unwrap_or(&0) as u8,
                        imm: ((*data.get("imm").unwrap_or(&0) as i32) << 11) >> 11
                    }))
                },
                _ => return None
            }
        })
        .collect::<Vec<DecodedInstr>>();

    Ok(expected)
}

fn run_test(test: &str) -> Result<(), io::Error> {
    let instructions = read_hex_instructions(test)?;
    let expected = read_expected_instructions(test)?;

    for (instruction, expected) in instructions.iter().zip(expected) {
        let decoded = decode_instruction(*instruction)
            .expect(&format!("Couldn't decode instruction 0x{:08x}", instruction));

        assert_eq!(decoded, expected);
    }

    Ok(())
}

#[test]
fn test_level1() {
    assert!(run_test("level1").is_ok());
}

#[test]
fn test_level2() {
    assert!(run_test("level2").is_ok());
}

#[test]
fn test_level3() {
    assert!(run_test("level3").is_ok());
}

#[test]
fn test_level4() {
    assert!(run_test("level4").is_ok());
}

#[test]
fn test_level5() {
    assert!(run_test("level5").is_ok());
}