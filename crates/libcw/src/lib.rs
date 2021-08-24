//! libcw is library designed to count words fast on any arch. It has **zero
//! dependencies** and compiles to blazing fast machine code
//! that outperforms `GNU's coreutils wc` engine on most situations, while
//! providing more features on Rust's safer & simpler code.
//!
//! To use `libcw` on your project, add this to your `Cargo.toml` file
//!
//! ```toml
//! [dependencies]
//!  libcw = { git="https://github.com/Altair-Bueno/cw" }
//! ```
//!
//!
//! # Features
//! - Platform agnostic
//! - Fast performance
//! - 100% Rust safe `std` code
//! - Zero dependencies, small size
//! - Selected encoding is used everywhere, even on `max line length`
//!
//! # Usage
//! To count words, you need some kind of [BufRead](std::io::BufRead) instance,
//! from which a Parser will read. To get started, set up your [Parser](crate::Parser)
//! instance with the desired configuration and call the `compute` method to
//! obtain the results
//!
//! ```no_run
//! # use libcw::Parser;
//! # use libcw::config::{Encoding, LineBreak};
//! # use std::io::BufReader;
//! # use std::fs::File;
//! # use std::io;
//! # fn main() -> io::Result<()> {
//! let parser = Parser::new(
//!     Encoding::UTF8,
//!     LineBreak::LF,
//!     // lines, words, chars, bytes, max-line-length
//!     true,true,true,true,true
//! );
//! let read = BufReader::new(File::open("foo.txt")?);
//! let stats_from_read = parser.proccess(read);
//! # Ok(())
//! # }
//! ```
//!
//! # Performance
//! See this repo [BENCH.md](https://github.com/Altair-Bueno/cw/blob/master/BENCH.md)
//! to learn more about this crate's performance

#![warn(missing_docs)]

pub use parser::Parser;
pub use stats::Stats;
pub mod config;
mod parser;
mod state;
mod stats;
