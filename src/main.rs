#![feature(let_chains)]
#![feature(generic_arg_infer)]
#![feature(transmutability)]

use assemble::assemble;
use cpu::Cpu;

mod assemble;
mod cpu;

/// Calculates 5 + 10*2 + 10*2
pub const ASSEMBLY: &str = concat!(
    "load 0 5 ",  // Load 5 into register 0
    "load 1 10 ", // Load 10 into register 1
    "call 5 ",    //Call add_twice  TODO: Function syntax
    "call 5 ",
    "exit ",
    // Function to add register 1 to register 0 twice
    "add 0 1 ",
    "add 0 1 ",
    "ret "
);

fn main() {
    let binary = assemble(ASSEMBLY);

    let mut cpu = Cpu::with_memory(&binary).unwrap();
    cpu.run();

    println!("5 + 10 * 2 + 10 * 2 = {}", cpu.data_registers[0]);
}
