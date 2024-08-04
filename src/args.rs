use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// File to read from
    #[arg(required = true)]
    pub file: String,

    /// Print debug info
    #[arg(short, long, default_value_t = false)]
    pub debug: bool,

    /// Don't print any info to stdout
    #[arg(short, long, default_value_t = false)]
    pub quiet: bool,
}
