use args::Args;
use clap::Parser;
use loader::Loader;

mod args;
mod asm;
mod lexer;
mod loader;

const BUILD_DIR: &str = ".skibidi";

fn main() {
    let args = Args::parse();

    // read file from args
    let file = std::fs::read_to_string(&args.file).expect("Failed to read file");

    let mut l = Loader::new(format!("Lexing {}...", args.file));
    let tokens = lexer::lex(&file);
    l.stop_success();

    if args.debug {
        for token in &tokens {
            println!("{:?}", token);
        }
    }

    // create build directory
    asm::create_build_dir(BUILD_DIR).unwrap_or_else(|err| {
        eprintln!("Failed to create build directory: {}", err);
        std::process::exit(1);
    });

    let asm_output = asm::tokens_to_asm(&tokens);

    if args.debug {
        println!("{}", asm_output);
    }

    asm::make_executable(&asm_output, &args.file);
}
