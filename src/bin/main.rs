use std::error::Error;
use std::fmt::Display;
use std::str::FromStr;

use bingrep::common::{DataType, Endianness};
use bingrep::filters::filter::create_filters;
use bingrep::scanner::Scanner;
use bingrep::workers::convertors::{FromBigEndian, FromLittleEndian};
use bingrep::workers::native_processor::NativeProcessor;
use clap::Parser;

/// Forensics grep.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Files to search
    #[arg(short, long)]
    file: String,

    #[clap(long = "data-type", short = 'd')]
    data_type: DataType,

    /// Minimum value to match
    #[arg(long, short = 'm', allow_hyphen_values = true)]
    min: Option<String>,

    /// Maximum value to match
    #[arg(long, short = 'M', allow_hyphen_values = true)]
    max: Option<String>,

    #[clap(value_enum, long = "endian", short = 'e', default_value_t = Endianness::Little)]
    endianess: Endianness,
}

fn parse_num<T: FromStr>(num: Option<String>) -> Result<Option<T>, <T as std::str::FromStr>::Err> {
    if num.is_none() {
        return Ok(None);
    }

    let converted = T::from_str(num.unwrap().as_str())?;
    return Ok(Some(converted));
}

fn run<T>(
    file_path: &str,
    min: Option<String>,
    max: Option<String>,
    endianness: Endianness,
) -> Result<(), Box<dyn Error>>
where
    // TODO(danilan): Fix this mess, do we really need this?s
    T: FromStr + Copy + PartialOrd + Display + 'static,
    T: FromLittleEndian<Output = T> + FromBigEndian<Output = T>,
    <T as std::str::FromStr>::Err: std::error::Error,
{
    let processor = NativeProcessor::<T>::new(endianness);

    let min = parse_num::<T>(min)?;
    let max = parse_num::<T>(max)?;

    let filter = create_filters(min, max);

    let mut grepper = Scanner::<T>::new(file_path, Box::new(processor), filter);
    grepper.scan()?;

    Ok(())
}

fn run_type(
    data_type: DataType,
    file_path: &str,
    min: Option<String>,
    max: Option<String>,
    endianness: Endianness,
) -> Result<(), Box<dyn Error>> {
    match data_type {
        DataType::I8 => run::<i8>(file_path, min, max, endianness),
        DataType::I16 => run::<i16>(file_path, min, max, endianness),
        DataType::I32 => run::<i32>(file_path, min, max, endianness),
        DataType::I64 => run::<i64>(file_path, min, max, endianness),
        DataType::I128 => run::<i128>(file_path, min, max, endianness),
        DataType::U8 => run::<u8>(file_path, min, max, endianness),
        DataType::U16 => run::<u16>(file_path, min, max, endianness),
        DataType::U32 => run::<u32>(file_path, min, max, endianness),
        DataType::U64 => run::<u64>(file_path, min, max, endianness),
        DataType::U128 => run::<u128>(file_path, min, max, endianness),
        DataType::F32 => run::<f32>(file_path, min, max, endianness),
        DataType::F64 => run::<f64>(file_path, min, max, endianness),
    }
}

fn main() {
    let args = Args::parse();

    run_type(
        args.data_type,
        args.file.as_str(),
        args.min,
        args.max,
        args.endianess,
    )
    .expect("should succeed");
}
