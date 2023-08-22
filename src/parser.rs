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

pub fn parse(source: &str) -> Result<Vec<Instruction>, String> {
    let tokens = source.chars();
    let mut instructions = Vec::new();
    let mut loop_stack = Vec::new();

    for token in tokens {
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
                    None => return Err("Unmatched ']'".to_string()),
                };
                instructions[open_pos] = Instruction::JumpIfZero(instructions.len());
                Instruction::JumpIfNotZero(open_pos)
            }
            _ => continue,
        };
        instructions.push(instruction);
    }
    if !loop_stack.is_empty() {
        return Err("Unmatched '['".to_string());
    }

    Ok(instructions)
}