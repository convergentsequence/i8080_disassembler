use std::fmt::Display;

use crate::opcodes;

#[derive(Clone)]
pub struct Instruction {
    pub opcode: opcodes::Opcode,
    pub arg: u16,
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.opcode.size {
            2 => {
                let mut replaced = self
                    .opcode
                    .name
                    .replace("D8", &format!("#0x{:02x}", self.arg));
                replaced = replaced.replacen(" ", " \t", 1);

                return write!(f, "{}", replaced);
            }
            3 => {
                let mut replaced = self
                    .opcode
                    .name
                    .replace("D16", &format!("#0x{:04x}", self.arg));

                replaced = replaced.replace("adr", &format!("${:04x}", self.arg));
                replaced = replaced.replacen(" ", " \t", 1);

                return write!(f, "{}", replaced);
            }
            _ => write!(f, "{}", self.opcode.name.replacen(" ", "\t", 1)),
        }
    }
}

pub fn classify(buffer: &Vec<u8>, pc: &usize) -> (Instruction, usize) {
    let opcode = &opcodes::OPCODES[buffer[*pc] as usize].clone();

    if opcode.size == 1 {
        return (
            Instruction {
                opcode: opcode.clone(),
                arg: 0,
            },
            1,
        );
    }

    let mut arg: u16 = 0;
    for i in (1..opcode.size).rev() {
        arg <<= 8;
        arg |= buffer[*pc + i as usize] as u16;
    }

    return (
        Instruction {
            opcode: opcode.clone(),
            arg,
        },
        opcode.size as usize,
    );
}
