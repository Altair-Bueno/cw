use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

use criterion::{Bencher, BenchmarkId, black_box, Criterion, criterion_group, criterion_main};

use libcw::config::{Encoding, LineBreak};
use libcw::Parser;

const WORLD192: &str = "resources/utf8/world192.txt";
const SAMPLE3: &str = "resources/utf8/sample3.txt";


pub fn load_file(path: &str) -> Vec<u8> {
    let mut temp = Vec::new();
    File::open(path).unwrap().read_to_end(&mut temp).unwrap();
    temp
}


pub fn criterion_benchmark(c: &mut Criterion) {
    let parsers = HashMap::from([
        ("Lines", Parser::new(Encoding::UTF8, LineBreak::LF, true, false, false, false, false)),
        ("Words", Parser::new(Encoding::UTF8, LineBreak::LF, false, true, false, false, false)),
        ("Chars", Parser::new(Encoding::UTF8, LineBreak::LF, false, false, true, false, false)),
        ("Bytes", Parser::new(Encoding::UTF8, LineBreak::LF, false, false, false, true, false)),
        ("Max length", Parser::new(Encoding::UTF8, LineBreak::LF, false, false, false, false, true)),
    ]);
    let data = load_file(WORLD192);

    for (k, v) in parsers.into_iter() {
        c.bench_with_input(
            BenchmarkId::new("Parser::process", k),
            &(v, data.as_slice()),
            |bencher, (parser, data)| bencher.iter(|| parser.process(black_box(*data))),
        );
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
