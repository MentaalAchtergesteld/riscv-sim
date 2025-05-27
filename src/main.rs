use components::{CPUError, CPU};
use stages::DecodeError;

mod tests;
mod util;
mod instruction_formats;
mod components;
mod stages;

fn main() {
    let mut cpu = CPU::new(1024*128, 1024*128);

    let bin = std::fs::read("testdata/programs/msg.bin").unwrap();
    cpu.load_elf(&bin).unwrap();

    // println!("{}", cpu.instr_mem);

    let mut msg = vec![];

    for _ in 0..1000 {
        println!("reg 13: {}", cpu.regs[13]);
        let result =  cpu.cycle();

        if let Err(err) = result {
            if let CPUError::DecodeError { source: DecodeError::EndOfProgram, pc: _ } = err {
                println!("End of program! Last PC: 0x{:08x}", cpu.pc.address);
            } else {
                eprintln!("CPU Error: {}", err)
            }

            break;
        }

        if let Some((addr, val)) = cpu.last_store {
            if addr == 0x200 {
                println!("val: {}", val);
                println!("char: {}", val as u8 as char);
                msg.push(val as u8 as char);
            }
        }
    }

    println!("69632: {}", cpu.data_mem.read_word(69632).unwrap());
    println!("69936: {}", cpu.data_mem.read_word(69936).unwrap());

    println!("---");
    println!("REGISTERS:");
    for (reg, data) in cpu.regs.iter().enumerate() {
        println!("0x{:08x}: {}", reg, *data as i32);
    }

    println!("{}", msg.iter().collect::<String>())
}
