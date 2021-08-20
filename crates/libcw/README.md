<!-- cargo-sync-readme start -->

libcw is library designed to count words fast on any platform, including
`wasm`. It has **zero dependencies** and compiles to blazing fast machine code
that outperforms `GNU's coreutils wc` engine on most situations, while
providing more features on Rust's safer & simpler code.


# Features
- Platform agnostic
- Fast performance
- 100% Rust safe `std` code
- Zero dependencies, small size
- Selected encoding is used everywhere, even on `max line length`

# Usage
To count words, you need some kind of [BufRead](std::io::BufRead) instance,
from which a Parser will read. To get started, set up your [Parser](crate::Parser)
instance with the desired configuration and call the `compute` method to
obtain the results

```rust
let parser = Parser::new(
    Encoding::UTF8,
    LineBreak::LF,
    // lines, words, chars, bytes, max-line-length
    true,true,true,true,true
);
let read = BufReader::new(File::open("foo.txt")?);
let stats_from_read = parser.proccess(read);
```

# Performance
See this repo [BENCH.md](https://github.com/Altair-Bueno/cw/blob/master/BENCH.md)
to learn more about this crate's performance

<!-- cargo-sync-readme end -->