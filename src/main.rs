use args::Args;
use clap::Parser;

mod args;
mod lexer;

fn main() {
    let args = Args::parse();

    // read file from args
    let file = std::fs::read_to_string(&args.file).expect("Failed to read file");
    let tokens = lexer::lex(&file);

    for token in tokens {
        println!("{:?}", token);
    }
}
