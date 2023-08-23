use std::io::{Read, Write, ErrorKind};

use crate::parser::Instruction;


#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Error {
    MemoryOutOfBounds,
    ReadError(String),
    WriteError(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use Error::*;
        match self {
            MemoryOutOfBounds => write!(f, "Memory out of bounds"),
            ReadError(e) => write!(f, "Read error: {}", e),
            WriteError(e) => write!(f, "Write error: {}", e),
        }
    }
}

impl std::error::Error for Error {}


pub fn run<R: Read, W: Write>(program: &[Instruction], mut input: R, mut output: W) -> Result<(), Error> {
    let mut memory = Memory::new(2_usize.pow(16));
    let mut ip = 0;

    while let Some(instruction) = program.get(ip) {
        ip += 1;
        match instruction {
            Instruction::IncCell => {
                memory.add_cell(1);
            }
            Instruction::DecCell => {
                memory.add_cell(-1);
            }
            Instruction::IncPtr => {
                memory.move_ptr(1)?;
            }
            Instruction::DecPtr => {
                memory.move_ptr(-1)?;
            }
            Instruction::JumpIfZero(pos) => {
                if memory.get_cell() == 0 {
                    ip = *pos;
                }
            }
            Instruction::JumpIfNotZero(pos) => {
                if memory.get_cell() != 0 {
                    ip = *pos;
                }
            }
            Instruction::Read => {
                memory.set_cell(read_byte(&mut input)?);
            }
            Instruction::Write => {
                write_byte(&mut output, memory.get_cell())?;
            }
        }
    }

    Ok(())
}

pub fn write_byte<W: Write>(mut w: W, byte: u8) -> Result<(), Error> {
    w.write_all(&[byte]).map_err(|e| Error::WriteError(e.to_string()))
}

pub fn read_byte<R: Read>(mut r: R) -> Result<u8, Error> {
    let mut buf = [0; 1];
    match r.read_exact(&mut buf) {
        Ok(_) => {},
        Err(ref e) if e.kind() == ErrorKind::UnexpectedEof => return Ok(0),
        Err(e) => return Err(Error::ReadError(e.to_string())),
    }
    Ok(buf[0])
}

struct Memory {
    cells: Vec<u8>,
    ptr: usize,
}

impl Memory {
    pub fn new(size: usize) -> Self {
        Self {
            cells: vec![0; size],
            ptr: 0,
        }
    }

    pub fn move_ptr(&mut self, offset: isize) -> Result<(), Error>{
        let new_ptr = self.ptr.checked_add_signed(offset)
            .ok_or(Error::MemoryOutOfBounds)?;
        if new_ptr >= self.cells.len() {
            return Err(Error::MemoryOutOfBounds);
        }
        self.ptr = new_ptr;
        Ok(())
    }

    pub fn add_cell(&mut self, val: i8) {
        let cell = self.cells.get_mut(self.ptr).unwrap();
        *cell = cell.wrapping_add(val as u8);
    }

    pub fn get_cell(&mut self) -> u8 {
        *self.cells.get(self.ptr).unwrap()
    }

    pub fn set_cell(&mut self, val: u8) {
        let cell = self.cells.get_mut(self.ptr).unwrap();
        *cell = val;
    }
}