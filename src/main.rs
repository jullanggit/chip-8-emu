use std::array;

#[derive(Default, Debug)]
struct Cpu {
    cur_op: u16,
    registers: [u8; 2],
}
impl Cpu {
    // TODO: Read from memory
    fn read_opcode(&self) -> u16 {
        self.cur_op
    }
    /// Runs one cpu cycle
    fn run(&mut self) {
        let opcode = self.read_opcode();

        // 0 = Opcode Group
        // 1 = Register x
        // 2 = Register y
        // 3 = Opcode Subgroup
        let decoded = array::from_fn(|index| {
            // Inverted to make following the tutorial easier
            let inverted_index = 3 - index;
            let offset = inverted_index * 4;
            (opcode & (0x000F << offset) >> offset) as u8
        });

        match decoded {
            [0x8, _, _, 0x4] => self.add_xy(decoded[1], decoded[2]),
            _ => todo!("opcode: {opcode:04x}"),
        }
    }
    // Adds register y to register x
    fn add_xy(&mut self, x: u8, y: u8) {
        self.registers[x as usize] += self.registers[y as usize];
    }
}

fn main() {
    let mut cpu = Cpu {
        cur_op: 0x8014, // 8 = 2 regs, 0 = reg0, 1 = reg1, 4 = additition
        registers: [5, 10],
    };

    cpu.run();

    assert_eq!(cpu.registers[0], 15);

    println!("5+10 = {}", cpu.registers[0]);
}
