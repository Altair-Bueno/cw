use clap::ArgMatches;

use crate::cw::stats::Stats;

/// Convenience struct and functions for pretty printing Stats
#[derive(Debug, Default)]
pub struct PrettyPrint {
    pub lines: bool,
    pub words: bool,
    pub characters: bool,
    pub bytes: bool,
    pub legth: bool,
}

impl PrettyPrint {
    /// Generates a conditional PrettyPrint instance used for pretty printing Stats
    pub fn from_clap(args: &ArgMatches) -> PrettyPrint {
        let lines = args.is_present("lines");
        let words = args.is_present("words");
        let characters = args.is_present("characters");
        let bytes = args.is_present("bytes");
        let legth = args.is_present("line_length");

        PrettyPrint {
            lines,
            words,
            characters,
            bytes,
            legth,
        }
    }
    /// Returns a String representation of the given Stats struct, but only
    /// includes the requested information
    pub fn print(&self, stats: &Stats, file: &str) -> String {
        let mut string = String::new();
        if self.lines {
            string = format!("{}\t{}", string, stats.lines)
        }
        if self.words {
            string = format!("{}\t{}", string, stats.words)
        }
        if self.characters {
            string = format!("{}\t{}", string, stats.characters)
        }
        if self.bytes {
            string = format!("{}\t{}", string, stats.bytes)
        }
        if self.legth {
            string = format!("{}\t{}", string, stats.legth)
        }
        if string.is_empty() {
            string = format!(
                "{}\t{}\t{}\t{}\t{}",
                stats.lines, stats.words, stats.characters, stats.bytes, stats.legth,
            )
        }
        format!("{}\t{}", string, file)
    }
}
