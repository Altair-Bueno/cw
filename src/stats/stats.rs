use std::fmt::{Display, Formatter};

/// Represents Stats for a file
#[derive(Debug, Default, Eq, PartialEq)]
pub struct Stats {
    pub lines: u32,
    pub words: u32,
    pub characters: u32,
    pub bytes: u32,
    //colums: Colums,
}

impl Stats {
    /// Creates a new Stats struct using the given parameters
    pub fn new(lines: u32, words: u32, characters: u32, bytes: u32) -> Stats {
        Stats {
            lines,
            words,
            characters,
            bytes,
        }
    }
    /// Combines two stats. Usefull when buffering a file. Consumes both
    /// arguments for improved performance. There is no need to
    /// de-referenciate or alloc more memory
    pub fn combine(mut self, s: Stats) -> Stats {
        self.lines += s.lines;
        self.words += s.words;
        self.characters += s.characters;
        self.bytes += s.bytes;
        self
    }
}

impl Display for Stats {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}\t{}\t{}\t{}",
            self.lines, self.words, self.characters, self.bytes
        )
    }
}
