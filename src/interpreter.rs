use std::io::{Read, Write};

use crate::parser::Instruction;


pub fn run<R: Read, W: Write>(program: &[Instruction], input: R, mut output: W) -> Result<(), String> {
    let mut memory = [0u8; 2_usize.pow(16)];
    let mut ptr = 0;
    let mut ip = 0;

    let mut input_bytes = input.bytes();

    while let Some(instruction) = program.get(ip) {
        ip += 1;
        match instruction {
            Instruction::IncCell => {
                memory[ptr] = memory[ptr].wrapping_add(1);
            }
            Instruction::DecCell => {
                memory[ptr] = memory[ptr].wrapping_sub(1);
            }
            Instruction::IncPtr => {
                ptr += 1;
            }
            Instruction::DecPtr => {
                ptr -= 1;
            }
            Instruction::JumpIfZero(pos) => {
                if memory[ptr] == 0 {
                    ip = *pos;
                }
            }
            Instruction::JumpIfNotZero(pos) => {
                if memory[ptr] != 0 {
                    ip = *pos;
                }
            }
            Instruction::Read => {
                let byte = input_bytes.next().unwrap_or(Ok(0)).map_err(|e| e.to_string())?;
                memory[ptr] = byte;
            }
            Instruction::Write => {
                output
                    .write_all(&[memory[ptr]])
                    .map_err(|e| e.to_string())?;
            }
        }
    }

    Ok(())
}