#[test]
fn test_rtype_decode() {
    use crate::RTYpe;
    
    let raw = 0x41FB8633;
    let rtype = RTYpe::from(raw);

    assert_eq!(rtype.opcode, 0b0110011);
    assert_eq!(rtype.rd,     0b01100);
    assert_eq!(rtype.funct3, 0b000);
    assert_eq!(rtype.rs1,    0b10111);
    assert_eq!(rtype.rs2,    0b11111);
    assert_eq!(rtype.funct7, 0b0100000);
}

#[test]
fn test_itype_decode() {
    use crate::IType;

    let raw = 0b111111111100_10111_110_01100_0010011;
    let itype = IType::from(raw);

    assert_eq!(itype.opcode, 0b0010011);
    assert_eq!(itype.rd,     0b01100);
    assert_eq!(itype.funct3, 0b110);
    assert_eq!(itype.rs1,    0b10111);
    assert_eq!(itype.imm,    0b111111111100);
}

#[test]
fn test_stype_decode() {
    use crate::SType;

    let raw: u32 = 0xFE752C23;
    let s = SType::from(raw);

    assert_eq!(s.opcode, 0b0100011);
    assert_eq!(s.funct3, 0b010);
    assert_eq!(s.rs1, 10); // x10
    assert_eq!(s.rs2, 7);  // x7
    assert_eq!(s.imm, -8); // signed immediate
}