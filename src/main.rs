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
    let source = std::fs::read_to_string(args.source).unwrap();
    let program = parser::parse(source.as_str()).unwrap();
    let input_string = args.input.unwrap_or_default();
    let input = input_string.as_bytes();
    let output = std::io::stdout();

    interpreter::run(&program, input, output).unwrap();
    println!();
}