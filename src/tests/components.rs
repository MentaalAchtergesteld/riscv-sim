use crate::components::*;

#[test]
fn test_program_counter_increment() {
    let mut pc = ProgramCounter::default();

    assert_eq!(pc.address, 0);
    pc.increment();
    assert_eq!(pc.address, 4)
}

#[test]
fn test_program_counter_set() {
    let mut pc = ProgramCounter::default();

    assert_eq!(pc.address, 0);
    pc.set(32);
    assert_eq!(pc.address, 32);
}

#[test]
fn test_memory_read_write() {
    let mut memory = Memory::new(24);
    
    memory.write_byte(0, 0xCF).expect("Couldn't write byte to address 0");
    assert_eq!(memory.read_byte(0, false).expect("Couldn't read byte from address 0"), 0xCF);
    assert_eq!(memory.read_byte(0, true).expect("Couldn't read byte from address 0") as i32, -0x31);

    memory.write_half_word(1, 0xA1B3).expect("Couldn't write half word to address 1");
    assert_eq!(memory.read_half_word(1, false).expect("Couldn't read half word from address 1"), 0xA1B3);
    assert_eq!(memory.read_half_word(1, true).expect("Couldn't read half word from address 1") as i32, -0x5E4D);

    memory.write_word(3, 0x12FD32AC).expect("Couldn't write word to address 3");
    assert_eq!(memory.read_word(3).expect("Couldn't read word from address 3"), 0x12FD32AC);

    memory.write_double_word(7, 0x3ACDA235E71BA258).expect("Couldn't write double word to address 7");
    assert_eq!(memory.read_double_word(7).expect("Couldn't read double word from address 7"), 0x3ACDA235E71BA258);
}

#[test]
fn test_memory_out_of_bounds() {
    let mut memory = Memory::new(1);

    assert!(memory.write_byte(1, 0xAA).is_err());
    assert!(memory.read_double_word(5).is_err());
}