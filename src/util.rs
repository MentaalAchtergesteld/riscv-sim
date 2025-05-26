pub fn extract_bits(instruction: u32, high: u8, low: u8) -> u32 {
    let width = high - low + 1;
    (instruction >> low) & ((1u32 << width) - 1)
}