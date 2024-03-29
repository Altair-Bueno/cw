use std::env::args;

use libcw::config::{Encoding, LineBreak};
use libcw::Parser;

/// This example shows how fast `libcw` can parse multiple files without
/// multithreading. To compile it run this on your commandline
///
/// ```bash
/// cargo run --features="tokio"  --example async_count_from_file
/// ```
///
/// you can find the example under `target/release/examples/async_count_from_file`
#[tokio::main]
pub async fn main() -> std::io::Result<()> {
    // Creates a parser with UTF8 encoding and LF linebreaks that returns
    // all stats (lines,words,chars,bytes and max length)
    let parser = Parser::new(Encoding::UTF8, LineBreak::LF, true, true, true, true, true);
    // Remove executable path
    let mut args = args();
    let _ = args.next();
    let start = std::time::Instant::now();

    // Process each file received as argument
    for f in args {
        let file = tokio::fs::File::open(f).await?;
        let buf_reader = tokio::io::BufReader::new(file);
        let stats = parser.process(buf_reader).await?;
        println!("{}", stats)
    }
    println!("Took {} ms", start.elapsed().as_millis());
    Ok(())
}
