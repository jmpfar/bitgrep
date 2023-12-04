use bingrep::common::Endianness;
use bingrep::scanner::Scanner;
use clap::Parser;

/// Forensics grep.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Files to search
    #[arg(short, long)]
    file: String,

    /// Minimum value to match
    #[arg(long, short = 'm', allow_hyphen_values = true)]
    min: Option<f64>,

    /// Maximum value to match
    #[arg(long, short = 'M', allow_hyphen_values = true)]
    max: Option<f64>,

    #[clap(value_enum, long = "endian", short = 'e')]
    endianess: Endianness,
}

fn main() {
    let args = Args::parse();

    let mut grepper: Scanner = Scanner::new(args.file, args.min, args.max, args.endianess);
    let _ = grepper.scan();
}
