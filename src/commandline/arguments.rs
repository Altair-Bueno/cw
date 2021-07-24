use crate::stats::Stats;
use clap::ArgMatches;

/// Convenience struct and functions for pretty printing Stats
#[derive(Debug, Default)]
pub struct Cwargs {
    pub lines: bool,
    pub words: bool,
    pub characters: bool,
    pub bytes: bool,
    // TODO linebreak Option<Enum>
}

impl Cwargs {
    /// Generates a conditional Cwargs instance used for pretty printing Stats
    pub fn from_clap(args: &ArgMatches) -> Cwargs {
        let lines = args.is_present("lines");
        let words = args.is_present("words");
        let characters = args.is_present("characters");
        let bytes = args.is_present("bytes");

        Cwargs {
            lines,
            words,
            characters,
            bytes,
        }
    }
    /// Returns a String representation of the given Stats struct, but only
    /// includes the requested information
    pub fn pretty_print_stats(&self, stats: &Stats) -> String {
        let mut changes = false;
        let mut string = String::new();

        if self.lines {
            changes = true;
            string = format!("{}\t{}", string, stats.lines)
        }
        if self.words {
            changes = true;
            string = format!("{}\t{}", string, stats.words)
        }
        if self.characters {
            changes = true;
            string = format!("{}\t{}", string, stats.characters)
        }
        if self.bytes {
            changes = true;
            string = format!("{}\t{}", string, stats.bytes)
        }
        if !changes {
            format!("{}", stats)
        } else {
            string
        }
    }
}
