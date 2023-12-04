use bingrep::common::Endianness;
use bingrep::scanner::Scanner;
use std::path::Path;

use criterion::{criterion_group, criterion_main, Criterion};

fn scanner_random_8k_benchmark(c: &mut Criterion) {
    const FILE_NAME : &str = "random.dat";

    let this_directory = Path::new(file!()).parent().unwrap();
    let path = this_directory.join(FILE_NAME);

    let mut scanner = Scanner::new(
        path.to_str().unwrap(),
        Some(30.1000),
        Some(32.12345),
        Endianness::Little,
    );

    c.bench_function(format!("scanner.scan() 8k random file [{FILE_NAME}]").as_str(), |b| {
        b.iter(|| scanner.scan().expect("should complete successfuly"));
    });
}

criterion_group!(benches, scanner_random_8k_benchmark);
criterion_main!(benches);
