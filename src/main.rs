#![feature(let_chains)]
#![feature(generic_arg_infer)]

#[derive(Debug)]
struct Cpu {
    // Last reg = carry flag (overflow)
    registers: [u8; 16],
    program_counter: u16,
    memory: [u16; 2048], // 4 KB of memory
}
impl Default for Cpu {
    fn default() -> Self {
        Self {
            registers: [0; _],
            program_counter: 0,
            memory: [0; _],
        }
    }
}
impl Cpu {
    fn read_opcode(&mut self) -> [u8; 2] {
        // to_le_bytes should just optimise out
        let opcode = self.memory[self.program_counter as usize].to_le_bytes();
        self.program_counter += 1;
        opcode
    }
    /// Run the cpu
    fn run(&mut self) {
        while let opcode = self.read_opcode()
            && opcode != [0, 0]
        {
            // 0 = Opcode Group
            // 1 = Register x
            // 2 = Register y
            // 3 = Opcode Subgroup
            let decoded = [
                (opcode[1] & 0xF0) >> 4,
                opcode[1] & 0x0F,
                (opcode[0] & 0xF0) >> 4,
                opcode[0] & 0x0F,
            ];

            match decoded {
                [0x8, _, _, 0x4] => self.add_xy(decoded[1], decoded[2]),
                _ => todo!("opcode: {:02x}{:02x}", opcode[0], opcode[1]),
            }
        }
    }
    // Adds register y to register x
    fn add_xy(&mut self, x: u8, y: u8) {
        let val_x = self.registers[x as usize];
        let val_y = self.registers[y as usize];

        let (result, overflow) = val_x.overflowing_add(val_y);

        self.registers[x as usize] = result;

        self.registers[15] = overflow as u8;
    }
}

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
