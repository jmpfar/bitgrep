use bitgrep::common::SourceFile;
use bitgrep::scanner::Scanner;
use bitgrep::workers::native_processor::NativeProcessor;
use bitgrep::{common::Endianness, filters::configuration::Configuration};
use std::fs::File;
use std::path::Path;

use criterion::{criterion_group, criterion_main, Criterion};

fn scanner_random_8k_minmax_benchmark(c: &mut Criterion) {
    const FILE_NAME: &str = "random.dat";

    let this_directory = Path::new(file!()).parent().unwrap();
    let path = this_directory.join(FILE_NAME);

    let configuration = Configuration {
        minimum: Some(30.1000),
        maximum: Some(35.12345),
        ..Default::default()
    };
    let filter = configuration.create_filter();
    let processor = NativeProcessor::new(Endianness::Little);
    let file = SourceFile::new(path.clone(), File::open(&path).unwrap());

    let mut scanner = Scanner::new(file, Box::new(processor), filter.unwrap());

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

    let configuration = Configuration {
        literal: Some(33.248462071692536),
        ..Default::default()
    };

    let filter = configuration.create_filter();
    let processor = NativeProcessor::new(Endianness::Little);
    let file = SourceFile::new(path.clone(), File::open(&path).unwrap());

    let mut scanner = Scanner::new(file, Box::new(processor), filter.unwrap());

    c.bench_function(
        format!("scanner.scan() literal 8k random file [{FILE_NAME}]").as_str(),
        |b| {
            b.iter(|| scanner.scan().expect("should complete successfuly"));
        },
    );
}

criterion_group!(
    benches,
    scanner_random_8k_minmax_benchmark,
    scanner_random_8k_literal_benchmark
);
criterion_main!(benches);
