# Deprecated. Use [wz](https://github.com/Altair-Bueno/wz) instead!

---

# cw

<!-- cargo-sync-readme start -->

cw (*count words*) is a faster alternative to classic GNU wc, written on pure
Rust. It provides the same tools as wc, but with a more friendly interface
and multiple encoding support. cw also provides its core
functionality as a library called `libcw` that can target any arch with no
platform-specific code. The Rust compiler leverages great performance with
stupidly simple source code

cw differentiates itself from other wc clones by providing great defaults. cw
will **always** count characters using the provided encoding, and thus, always
providing the right count. Other word counters will provide, for example, wrong
max line length on UTF-8 encoded text

To learn more about this project, visit it's [GitHub repo](https://github.com/Altair-Bueno/cw)


<!-- cargo-sync-readme end -->

![img.png](.github/readme/img.png)

# Install cw

## Build and install cw from source (recommended)

Because cw is written entirely on Rust, is as simple as using `cargo`. If you
already have installed
[`cargo`](https://doc.rust-lang.org/cargo/getting-started/installation.html) on
your system, run the following from the commandline:

```bash
# Ensure rust's toolchain is up-to-date
rustup update stable
git clone https://github.com/Altair-Bueno/cw.git
cd cw
cargo install --locked --path crates/cw
```

> Warning: This will install cw on `$HOME/.cargo/bin`. Ensure this location is
> on your shell's `$PATH` variable by running `echo $PATH | grep '.cargo/bin'`

### Completions

Shell completions for Zsh, Bash, Fish, Elvish and PowerShell can be found under
`target/release/build/cw-*/out`

```bash
# zsh shell
cp target/release/build/cw-*/out/completions/zsh/* /usr/local/share/zsh/site-functions
# Fish
cp target/release/build/cw-*/put/completions/fish/* /usr/local/share/fish/completions
```

## Pre compiled artifacts

1. Download the artifact that matches your OS and architecture from the 
   [releases page](https://github.com/Altair-Bueno/cw/releases/latest)
2. Unzip the archive
3. Move the binary to the desired destination folder. Make sure that your 
   shell's `PATH` includes said folder

# Options

The same functionality you'll expect from GNU wc, but with some extras. To see
the full list of options, type `cw -h` or `cw --help`:

## `tokio`

cw uses the high-performant library [`tokio`](https://tokio.rs/) for IO
concurrency. This allows `cw` to parse a file while the operating system is
loading another one.

You can use the `--multithread` flag to force the multithread runtime flavour
from tokio. This is useful when you want `cw` to use all CPU cores for heavy
workloads

> Bonus: `alias cm='cw --multithread'` for _count multithread_

## Different encodings and Linebreaks

By default, cw will search for UTF-8 encoded text, with LF (`U+000A`) line
breaks. Note that this crate **does not** validate any input. It asumes it's
encoded correctly, although invalid encoded input is safely managed

# Features

To use any of these features, add them to the `--features "..."` list. For
example:

```bash
cargo install --git https://github.com/Altair-Bueno/cw.git --features "mimalloc"
```

- `mimalloc`: Uses [mimalloc](https://github.com/microsoft/mimalloc) instead

# Performance

See [BENCH.md](BENCH.md)

# Wishlist

- Full Unicode support (eg process Z҉͈͓͈͎a̘͈̠̭l̨̯g̶̬͇̭o̝̹̗͎̙ ͟t͖̙̟̹͇̥̝͡e̥͘x͚̺̭̻͘t͉͔̩̲̘ correctly)
- UTF-16 encoding
- Auto-detect file encoding
- Make cw faster
