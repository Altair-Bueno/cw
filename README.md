# cw
cw *(count words)* is a modern alternative to classic wc, written on pure
Rust. It provides the same tools as wc, but with some extras, such as 
multithreading and different encoding support. Please note it is not the 
fastest word counter out there, althought it's pretty lightweight and 
doesn't depend on any platform-agnostic hardware. I just made it for learning
Rust

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
The same functionality you'll expect from GNU wc, but with some extras. To see
the full list of options, type `cw -h` or `cw --help`:

## Multithreading
When provided multiple files, cw can process them sequentially or in parallel
by providing the `-t N` option. It will use N additional threads to process all 
files

## Different encodings and Linebreaks
By default, cw will search for UTF-8 encoded text, with LF (`U+000A`) line 
breaks. You can choose another any other combination if you need it

# Why?
- I'm learning Rust
- My macOS version of wc cannot count UTF-8 encoded characters correctly, or 
  at least i don't know how to use it
- Set up using enviroment variables is painfull if you just want to read one
  single UTF-8 file
- Because i can

# Wishlist

- Pre-compiled binaries using GitHub workflows
- Full unicode support eg proccess Z҉͈͓͈͎a̘͈̠̭l̨̯g̶̬͇̭o̝̹̗͎̙ ͟t͖̙̟̹͇̥̝͡e̥͘x͚̺̭̻͘t͉͔̩̲̘ correctly
- UTF-16 encoding
- Auto detect file encoding
- Colored output
- Make cw faster
