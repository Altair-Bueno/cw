# cw
cw *(count words)* is a modern alternative to classic wc, written on pure
Rust. It provides the same tools as wc, but with some extras, such as 
multithreading and different encoding support. cw also provides its core 
functionality as a library that can target any platform (including wasm), with 
no platform-specific code. Rustc leverages great performance on any platform 
with stupidly simple source code

![img.png](.github/readme/img.png)

# Build and install

Because cw is written entiretly on Rust, is as simple as using `cargo`. To 
build cw, run the folowing on the commandline:

```bash
git clone https://github.com/Altair-Bueno/cw
# Install latest release
cargo install --path cw/
```

This will install cw on `$HOME/.cargo/bin`. Ensure this location is on your
shell's `$PATH` variable

# Uninstalling

Run this from the commandline

```bash
cargo uninstall cw
```

# Features
The same functionality you'll expect from GNU wc, but with some extras. To see
the full list of options, type `cw -h` or `cw --help`:

## Multithreading
When provided multiple files, cw can process them sequentially or in parallel
by providing the `-t <n>` option. It will use n additional threads to process all 
files

## Different encodings and Linebreaks
By default, cw will search for UTF-8 encoded text, with LF (`U+000A`) line 
breaks. Note that this crate **does not** validate any UTF-8. It asumes it's 
encoded correctly, althought invalid UTF-8 is safely managed

# Performance
See [BENCH.md](BENCH.md)

# Wishlist

- Full unicode support (eg proccess Z҉͈͓͈͎a̘͈̠̭l̨̯g̶̬͇̭o̝̹̗͎̙ ͟t͖̙̟̹͇̥̝͡e̥͘x͚̺̭̻͘t͉͔̩̲̘ correctly)
- UTF-16 encoding
- Auto-detect file encoding
- Make cw faster