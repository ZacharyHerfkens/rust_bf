mod interpreter;
mod parser;

use clap::Parser;

/// A simple brainfuck interpreter written in Rust.
#[derive(Parser)]
struct BFApp {
    /// The brainfuck source file to run
    source: String,

    /// An optional input string to use, defaults to stdin
    #[arg(short, long)]
    input: Option<String>,

    /// The amount of memory the brainfuck program is given in bytes
    #[arg(short = 'm', long)]
    max_memory: Option<usize>,

    /// The amount of steps the interpreter is allowed to run, default is unbounded
    #[arg(short = 's', long)]
    max_steps: Option<usize>,
}

fn main() {
    let args = BFApp::parse();
    let output = std::io::stdout();

    let source = match std::fs::read_to_string(&args.source) {
        Ok(source) => source,
        Err(e) => {
            eprintln!("File Error: {}", e);
            return;
        }
    };

    let program = match parser::parse(&source) {
        Ok(program) => program,
        Err(e) => {
            eprintln!("Parsing Error: {}", e);
            return;
        }
    };

    let input: Box<dyn std::io::Read> = args.input
        .map(|input| Box::new(std::io::Cursor::new(input)) as Box<dyn std::io::Read>)
        .unwrap_or_else(|| Box::new(std::io::stdin()));

    let settings = interpreter::Settings {
        max_memory: args.max_memory.unwrap_or(2_usize.pow(16)),
        max_steps: args.max_steps,
        input,
        output,
    };

    if let Err(e) = interpreter::run(&program, settings) {
        eprintln!("Interpreter Error: {}", e);
        return;
    }

    println!();
}