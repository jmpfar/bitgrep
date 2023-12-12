use std::cell::RefCell;
use std::error::Error;
use std::fs::File;
use std::io::{self, stdin, IsTerminal};
use std::path::PathBuf;
use std::rc::Rc;
use std::str::FromStr;

use bitgrep::common::{DataType, Endianness, SourceFile, DEFAULT_BUFFER_SIZE};
use bitgrep::filters::configuration::{Configuration, EntropyConfig};
use bitgrep::scanner::Scanner;
use bitgrep::types::compare::Compare;
use bitgrep::workers::entropy_processor::EntropyProcessor;
use bitgrep::workers::native_processor::NativeProcessor;
use bitgrep::workers::processors::Processor;
use clap::error::ErrorKind::InvalidValue;
use clap::error::{ContextKind, ContextValue};
use clap::CommandFactory;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[clap(group(
    clap::ArgGroup::new("required-filters")
        .required(true)
        .multiple(true)
        .args(&["min", "max", "literal"]),
))]
struct Args {
    /// Path to file, use - to read from stdin (must not be a tty)
    #[arg(short, long)]
    file: PathBuf,

    /// Data type of value in rust numerical data types
    #[clap(long = "data-type", short = 'd')]
    data_type: DataType,

    /// Minimum value to match
    #[arg(
        long,
        short = 'm',
        allow_hyphen_values = true,
        conflicts_with = "literal"
    )]
    min: Option<String>,

    /// Maximum value to match
    #[arg(
        long,
        short = 'M',
        allow_hyphen_values = true,
        conflicts_with = "literal"
    )]
    max: Option<String>,

    /// A specific value to match
    #[arg(long,
        short = 'l',
        allow_hyphen_values = true,
        conflicts_with_all= ["min", "max"],
        long_help = "A specific value to match.
In floating point datatypes, this uses a ULPS of 4 to detect floating point values which are approximately equal"
)]
    literal: Option<String>,

    /// Filters by maximum entropy
    #[arg(
        long,
        allow_hyphen_values = true,
        long_help = "Filters by maximum entropy. Entropy is calculated by the 4k preceeding the detected value.
An entropy over 7.5 is usually considered encrypted/random data."
    )]
    max_entropy: Option<f64>,

    /// Endianness of searched value
    #[clap(value_enum, long = "endian", short = 'e', default_value_t = Endianness::Little)]
    endianness: Endianness,
}

fn parse_num<T: FromStr>(num: Option<String>) -> Option<T> {
    let num = num?;

    let converted = T::from_str(num.as_str());
    if converted.is_err() {
        let mut err = Args::command().error(InvalidValue, "Failed parsing number");
        err.insert(ContextKind::InvalidValue, ContextValue::String(num));
        err.exit();
    }

    return converted.ok();
}

fn open_file<'a>(path: PathBuf) -> Result<SourceFile<'a>, Box<dyn Error>> {
    if path == PathBuf::from("-") {
        if io::stdin().is_terminal() {
            let err =
                Args::command().error(InvalidValue, "using stdin is not supported in TTY mode");
            let _ignore = err.print();
            Args::command().print_help().unwrap();
            ::std::process::exit(2);
        }

        return Ok(SourceFile::new(PathBuf::from("<stdin>"), stdin().lock()));
    }

    let file = File::open(&path)?;
    return Ok(SourceFile::new(path, file));
}

fn run<T>(args: &Args) -> Result<(), Box<dyn Error>>
where
    T: Compare + 'static,
    <T as std::str::FromStr>::Err: std::error::Error,
{
    let processor = NativeProcessor::<T>::new(args.endianness);

    let min = parse_num::<T>(args.min.clone());
    let max = parse_num::<T>(args.max.clone());
    let literal = parse_num(args.literal.clone());

    // TODO(danilan): unite all buffer size usages to a single place
    let entropy_producer = args.max_entropy.map(|_| {
        Rc::new(RefCell::new(EntropyProcessor::<T>::new(
            DEFAULT_BUFFER_SIZE,
        )))
    });

    let entropy_config = args.max_entropy.map(|max| EntropyConfig {
        max_entropy: max,
        entropy_producer: entropy_producer.as_ref().unwrap().clone(),
    });

    let config = Configuration {
        literal,
        minimum: min,
        maximum: max,
        entropy: entropy_config,
    };

    let filter = config.create_filter().ok_or("Failed creating filters")?;

    // Unwrap option to coerce type, hell on earth
    let entropy_processor = entropy_producer.map(|rc| rc as Rc<RefCell<dyn Processor<T>>>);

    let file = open_file(args.file.clone())?;

    let mut scanner =
        Scanner::<T>::with_entropy_processor(file, Box::new(processor), filter, entropy_processor);
    scanner.scan()?;

    Ok(())
}

fn run_type(data_type: DataType, args: &Args) -> Result<(), Box<dyn Error>> {
    match data_type {
        DataType::I8 => run::<i8>(args),
        DataType::I16 => run::<i16>(args),
        DataType::I32 => run::<i32>(args),
        DataType::I64 => run::<i64>(args),
        DataType::I128 => run::<i128>(args),
        DataType::U8 => run::<u8>(args),
        DataType::U16 => run::<u16>(args),
        DataType::U32 => run::<u32>(args),
        DataType::U64 => run::<u64>(args),
        DataType::U128 => run::<u128>(args),
        DataType::F32 => run::<f32>(args),
        DataType::F64 => run::<f64>(args),
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    return run_type(args.data_type.clone(), &args);
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Args::command().debug_assert();
}
