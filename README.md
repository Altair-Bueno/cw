# cw
cw *(count words)* is a modern alternative to classic GNU wc, written on pure
Rust. It provides the same tools as GNU wc, but with some extras, such as 
multithreading and different encoding support

![img.png](.github/readme/img.png)

# Build and install

Because cw is written entiretly on Rust, is as simple as using `cargo`. To 
build and install cw, run the folowing on the commandline:

```bash
# NOTE: Installing might take some time
git clone https://github.com/Altair-Bueno/cw
cargo install --path ./cw
```

This will install cw on `$HOME/.cargo/bin`. Ensure this location is on your
shell's `$PATH` variable

# Uninstalling

Run this from the commandline

```bash
cargo uninstall cw
```

# Features
The same functionality you'll expect from GNU wc, but with some extras:

## Multithreading
When provided multiple files, cw can process them sequentially or in parallel
by providing the `-t N` option. It will use N additional threads to process all 
files

## Different encodings and Linebreaks
By default, cw will search for UTF-8 encoded text, with LF (`\n`) line breaks.
You can choose another any other combination if you need it

# Wishlist

- Pre compiled binaries using GitHub workflows
- More encodings: UTF-16 isn't done yet
- Auto detect file encoding
- Make cw faster