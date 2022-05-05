<!-- cargo-sync-readme start -->

libcw is library designed to count words fast on any arch. It has **zero
dependencies** and compiles to blazing fast machine code
that outperforms `GNU's coreutils wc` engine on most situations, while
providing more features on Rust's safer & simpler code.

To use `libcw` on your project, add this to your `Cargo.toml` file

```toml
[dependencies]
 libcw = { git="https://github.com/Altair-Bueno/cw" }
```


# Spotlights
- Platform agnostic
- Fast performance
- 100% Rust safe `std` code
- Zero dependencies, small size
- Selected encoding is used everywhere, even on `max line length`

# Usage
`libcw`'s API is exposed through the [`Parser`](crate::Parser) module

# Performance
See this repo [BENCH.md](https://github.com/Altair-Bueno/cw/blob/master/BENCH.md)
to learn more about this crate's performance

# Feature flags
Although this crate is designed for minimal size and high throughput, a set
of features is provided for crate compatibility

- `tokio`: Enables async [Parser process](crate::Parser::process\(\)) for
the tokio runtime
- `serde`: Enables serde serialization of [Stats](crate::Stats)

<!-- cargo-sync-readme end -->

# Build documentation

Run this from the commandline to build and open this library's documentation

```bash
cargo doc --package libcw --open
```

# Examples

See [examples folder](examples)


