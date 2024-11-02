pub fn assemble(assembly: &str) -> Vec<u16> {
    let mut parts = assembly.split_whitespace();

    let mut assembled = Vec::new();

    while let Some(part) = parts.next() {
        let assembled_part = match part {
            "load" => {
                let reg: u16 = parts
                    .next()
                    .expect("Expected Register")
                    .parse()
                    .expect("Failed to parse register");
                let num: u16 = parts
                    .next()
                    .expect("Expected number")
                    .parse()
                    .expect("Failed to parse number");

                0x6000 | (reg << 8) | num
            }
            "call" => {
                let num: u16 = parts
                    .next()
                    .expect("Expected number")
                    .parse()
                    .expect("Failed to parse number");

                0x2000 | num
            }
            "add" => {
                let reg_1: u16 = parts
                    .next()
                    .expect("Expected Register")
                    .parse()
                    .expect("Failed to parse register");
                let reg_2: u16 = parts
                    .next()
                    .expect("Expected Register")
                    .parse()
                    .expect("Failed to parse register");

                0x8004 | (reg_1 << 8) | (reg_2 << 4)
            }
            "ret" => 0x00EE,
            "exit" => 0,
            other => todo!("Unsupported opcode: {other}"),
        };
        assembled.push(assembled_part);
    }
    assembled
}
