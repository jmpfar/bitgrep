use clap::Parser;
use grepper::DoubleGrepper;
mod filebuffer;
mod grepper;
mod hex;

/// Forensics grep.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Files to search
    #[arg(short, long)]
    file: String,

    /// Minimum value to match
    #[arg(long, short = 'm', allow_hyphen_values = true)]
    min: f64,

    /// Maximum value to match
    #[arg(long, short = 'M', allow_hyphen_values = true)]
    max: f64,

    #[arg(long, action)]
    float: bool,
}

fn main() {
    let args = Args::parse();

    let mut grepper: DoubleGrepper = DoubleGrepper::new(args.file, args.min, args.max, args.float);
    let _ = grepper.scan();
}
