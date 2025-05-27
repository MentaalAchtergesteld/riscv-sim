use components::CPU;

mod tests;
mod util;
mod instruction_formats;
mod components;
mod stages;

fn main() {
    let mut cpu = CPU::new(64*32, 512*32);

    let result = cpu.cycle();

    if let Err(error) = result {
        println!("Error: {:?}", error);
    }
}
