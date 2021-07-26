use clap::ArgMatches;

use crate::stats::stats::Stats;

/// Convenience struct and functions for pretty printing Stats
#[derive(Debug, Default)]
pub struct PrettyPrint {
    pub lines: bool,
    pub words: bool,
    pub characters: bool,
    pub bytes: bool,
    // TODO max colum size
}

impl PrettyPrint {
    /// Generates a conditional PrettyPrint instance used for pretty printing Stats
    pub fn from_clap(args: &ArgMatches) -> PrettyPrint {
        let lines = args.is_present("lines");
        let words = args.is_present("words");
        let characters = args.is_present("characters");
        let bytes = args.is_present("bytes");

        PrettyPrint {
            lines,
            words,
            characters,
            bytes,
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
        if string.len() == 0 {
            string = format!(
                "{}\t{}\t{}\t{}",
                stats.lines, stats.words, stats.characters, stats.bytes
            )
        }
        format!("{}\t{}", string, file)
    }
}
