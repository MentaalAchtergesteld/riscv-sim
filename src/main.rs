use components::{CPUError, CPU};
use stages::DecodeError;

mod tests;
mod util;
mod instruction_formats;
mod components;
mod stages;

fn main() {
    let mut cpu = CPU::new(196, 128);

    let bin = std::fs::read("testdata/programs/basic.bin").unwrap();
    cpu.load_elf(&bin).unwrap();

    println!("{}", cpu.instr_mem);

    for _ in 0..1000 {
        let result =  cpu.cycle();

        if let Err(CPUError::DecodeError { source: DecodeError::EndOfProgram, pc: _ }) = result {
            println!("End of program! Last PC: 0x{:08x}", cpu.pc.address);
            break;
        }
    }

    for (reg, data) in cpu.regs.iter().enumerate() {
        println!("0x{:08x}: {}", reg, *data as i32);
    }
}
