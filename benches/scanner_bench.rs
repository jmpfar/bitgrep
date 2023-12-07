use bitgrep::common::Endianness;
use bitgrep::filters::filter::create_filters;
use bitgrep::scanner::Scanner;
use bitgrep::workers::native_processor::NativeProcessor;
use std::path::Path;

use criterion::{criterion_group, criterion_main, Criterion};

fn scanner_random_8k_minmax_benchmark(c: &mut Criterion) {
    const FILE_NAME: &str = "random.dat";

    let this_directory = Path::new(file!()).parent().unwrap();
    let path = this_directory.join(FILE_NAME);

    let filter = create_filters(Some(30.1000), Some(32.12345), None);
    let processor = NativeProcessor::new(Endianness::Little);

    let mut scanner = Scanner::new(path.to_str().unwrap(), Box::new(processor), filter);

    c.bench_function(
        format!("scanner.scan() minmax 8k random file [{FILE_NAME}]").as_str(),
        |b| {
            b.iter(|| scanner.scan().expect("should complete successfuly"));
        },
    );
}

fn scanner_random_8k_literal_benchmark(c: &mut Criterion) {
    const FILE_NAME: &str = "random.dat";

    let this_directory = Path::new(file!()).parent().unwrap();
    let path = this_directory.join(FILE_NAME);

    let filter = create_filters(None, None, Some(30.178249837215606));
    let processor = NativeProcessor::new(Endianness::Little);

    let mut scanner = Scanner::new(path.to_str().unwrap(), Box::new(processor), filter);

    c.bench_function(
        format!("scanner.scan() literal 8k random file [{FILE_NAME}]").as_str(),
        |b| {
            b.iter(|| scanner.scan().expect("should complete successfuly"));
        },
    );
}



criterion_group!(benches, scanner_random_8k_minmax_benchmark, scanner_random_8k_literal_benchmark);
criterion_main!(benches);
