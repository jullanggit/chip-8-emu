#![feature(let_chains)]
#![feature(generic_arg_infer)]
#![feature(transmutability)]

use cpu::Cpu;

mod cpu;


fn main() {
    let mut cpu = Cpu::default();

    cpu.memory[0] = 0x8014; // 8 = 2 regs, 0 = reg0, 1 = reg1, 4 = additition
    cpu.memory[1] = 0x8024;
    cpu.memory[2] = 0x8034;
    cpu.memory[3] = 0x8044;
    cpu.memory[4] = 0x8054;

    cpu.registers[1] = 15;
    cpu.registers[2] = 10;
    cpu.registers[3] = 7;
    cpu.registers[4] = 32;
    cpu.registers[5] = 2;

    cpu.run();

    assert_eq!(cpu.registers[0], 15 + 10 + 7 + 32 + 2);

    println!("15 + 10 + 7 + 32 + 2 = {}", cpu.registers[0]);
}
