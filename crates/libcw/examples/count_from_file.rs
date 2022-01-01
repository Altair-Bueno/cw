use std::env::args;
use std::fs::File;
use std::io::BufReader;

use libcw::config::{Encoding, LineBreak};
use libcw::Parser;

/// This example shows how fast `libcw` can parse multiple files without
/// multithreading. To compile it run this on your commandline
///
/// ```bash
/// cargo build --example count_from_file --release
/// ```
///
/// you can find the example under `target/release/examples/count_from_file`
#[cfg(not(feature = "tokio"))]
fn main() -> std::io::Result<()> {
    // Creates a parser with UTF8 encoding and LF linebreaks that returns
    // all stats (lines,words,chars,bytes and max length)
    let parser = Parser::new(Encoding::UTF8, LineBreak::LF, true, true, true, true, true);
    // Remove executable path
    let mut args = args();
    let _ = args.next();
    let start = std::time::Instant::now();

    // Process each file received as argument
    for f in args {
        let buf_reader = BufReader::new(File::open(f)?);
        let stats = parser.process(buf_reader)?;
        println!("{}", stats)
    }
    println!("Took {} ms", start.elapsed().as_millis());
    Ok(())
}
#[cfg(feature = "tokio")]
pub fn main() {
    println!("Invalid feature")
}
