use std::{
    mem::{Assume, TransmuteFrom},
    panic,
};

pub struct Cpu {
    // Last reg = carry flag (overflow)
    pub data_registers: [u8; 16],
    address_register: u16, // The 'I' register
    program_counter: u16,
    pub memory: [u16; 2048], // 4 KB of memory
    stack: [u16; 16],
    stack_pointer: u8,
}
impl Default for Cpu {
    fn default() -> Self {
        Self {
            data_registers: [0; _],
            address_register: 0,
            program_counter: 0,
            memory: [0; _],
            stack: [0; _],
            stack_pointer: 0,
        }
    }
}
impl Cpu {
    pub fn with_memory(memory: &[u16]) -> Option<Self> {
        assert!(memory.len() <= 2048);

        let mut array = [0; _];
        array[..memory.len()].copy_from_slice(memory);

        Some(Self {
            memory: array,
            ..Default::default()
        })
    }
    fn read_opcode(&mut self) -> u16 {
        // to_le_bytes should just optimise out
        let opcode = self.memory[self.program_counter as usize];
        self.program_counter += 1;
        opcode
    }
    /// Run the cpu
    pub fn run(&mut self) {
        while let opcode = self.read_opcode()
            && opcode != 0
        {
            let bytes: [u8; 2] =
            // Is actually safe, as the Assume::NOTHING guarantees that all proof obligations
            // belong to the compiler
                unsafe { TransmuteFrom::<_, { Assume::NOTHING }>::transmute(opcode) };

            // 0 = Opcode Group
            // 1 = Register x
            // 2 = Register y
            // 3 = Opcode Subgroup
            let decoded = [
                (bytes[1] & 0xF0) >> 4,
                bytes[1] & 0x0F,
                (bytes[0] & 0xF0) >> 4,
                bytes[0] & 0x0F,
            ];

            match decoded {
                [2, _, _, _] => self.call_fn(opcode & 0x0FFF),
                [0, 0, 0xE, 0xE] => self.return_fn(),
                [6, reg, _, _] => self.store_num(reg, bytes[0]),
                // Operations involving two registers
                [8, reg_1, reg_2, subgroup] => match subgroup {
                    4 => self.add_reg(reg_1, reg_2),
                    _ => todo!("opcode: {:04x}", opcode),
                },
                _ => todo!("opcode: {:04x}", opcode),
            }
        }
    }
    // Adds register y to register x
    fn add_reg(&mut self, x: u8, y: u8) {
        let val_x = self.data_registers[x as usize];
        let val_y = self.data_registers[y as usize];

        let (result, overflow) = val_x.overflowing_add(val_y);

        self.data_registers[x as usize] = result;

        self.data_registers[15] = overflow as u8;
    }
    fn call_fn(&mut self, addr: u16) {
        if self.stack_pointer > self.stack.len() as u8 {
            panic!("Stack Overflow!")
        }

        self.stack[self.stack_pointer as usize] = self.program_counter;
        self.stack_pointer += 1;

        self.program_counter = addr;
    }
    fn return_fn(&mut self) {
        if self.stack_pointer == 0 {
            panic!("Stack Underflow!")
        }
        self.stack_pointer -= 1;
        self.program_counter = self.stack[self.stack_pointer as usize];
    }
    fn store_num(&mut self, reg: u8, num: u8) {
        self.data_registers[reg as usize] = num;
    }
}
