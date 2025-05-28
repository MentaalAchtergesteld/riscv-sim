use std::{env, fs};

use cpu::{CPU, CPUError, DecodeError};

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let program_path = &args[1];
    let full_path = env::current_dir().unwrap().join(program_path);

    let bytes = fs::read(full_path).expect("Failed to read program");

    let mut cpu = CPU::new(5012*32);
    cpu.load_elf(&bytes).unwrap();

    loop {
        let result = cpu.cycle();

        if let Err(error) = result {
            if let CPUError::DecodeError { source: DecodeError::EndOfProgram, pc } = error {
                println!("Program ended at PC 0x{:08x}.", pc);
            }

            break;
        }
    }
}
