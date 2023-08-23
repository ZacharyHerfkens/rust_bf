pub enum Instruction {
    IncCell,
    DecCell,
    IncPtr,
    DecPtr,
    JumpIfZero(usize),
    JumpIfNotZero(usize),
    Read,
    Write,
}

#[derive(Debug)]
pub enum Error {
    UnmatchedOpenLoop(usize, usize),
    UnmatchedCloseLoop(usize, usize),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use Error::*;
        match self {
            UnmatchedOpenLoop(line, col) => write!(f, "Unmatched '[' at line {}, col {}", line, col),
            UnmatchedCloseLoop(line, col) => write!(f, "Unmatched ']' at line {}, col {}", line, col),
        }
    }
}

impl std::error::Error for Error {}

pub fn parse(source: &str) -> Result<Vec<Instruction>, Error> {
    let mut tokens = CharCounter::new(source.chars());
    let mut instructions = Vec::new();
    let mut loop_stack = Vec::new();

    for token in tokens.by_ref() {
        let instruction = match token {
            '>' => Instruction::IncPtr,
            '<' => Instruction::DecPtr,
            '+' => Instruction::IncCell,
            '-' => Instruction::DecCell,
            '.' => Instruction::Write,
            ',' => Instruction::Read,
            '[' => {
                loop_stack.push(instructions.len());
                Instruction::JumpIfZero(0)
            }
            ']' => {
                let open_pos = match loop_stack.pop() {
                    Some(pos) => pos,
                    None => return Err(Error::UnmatchedCloseLoop(tokens.line(), tokens.col())),
                };
                instructions[open_pos] = Instruction::JumpIfZero(instructions.len());
                Instruction::JumpIfNotZero(open_pos)
            }
            _ => continue,
        };
        instructions.push(instruction);
    }
    if !loop_stack.is_empty() {
        return Err(Error::UnmatchedOpenLoop(tokens.line(), tokens.col()));
    }

    Ok(instructions)
}

struct CharCounter<I: Iterator<Item = char>> {
    chars: I,
    line: usize,
    col: usize,
}

impl<I: Iterator<Item = char>> CharCounter<I> {
    fn new(chars: I) -> Self {
        Self { chars, line: 1, col: 0 }
    }

    fn next(&mut self) -> Option<char> {
        let c = self.chars.next()?;
        if c == '\n' {
            self.line += 1;
            self.col = 0;
        } else {
            self.col += 1;
        }
        Some(c)
    } 

    fn line(&self) -> usize {
        self.line
    }

    fn col(&self) -> usize {
        self.col
    }
}

impl<I: Iterator<Item = char>> Iterator for CharCounter<I> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        self.next()
    }
}