mod interpreter;
mod parser;

use clap::Parser;

#[derive(Parser)]
struct BFApp {
    source: String,

    #[arg(short, long)]
    input: Option<String>,
}

fn main() {
    let args = BFApp::parse();
    let input_string = args.input.unwrap_or_default();
    let input = input_string.as_bytes();
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

    if let Err(e) = interpreter::run(&program, input, output) {
        eprintln!("Interpreter Error: {}", e);
        return;
    }

    println!();
}