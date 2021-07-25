use std::fmt::{Display, Formatter};
use std::ops::Add;



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
    /// Combines two stats. Usefull when buffering a file
    pub fn combine(&self, s: &Stats) -> Stats {
        Stats {
            lines: self.lines + s.lines,
            words: self.words + s.words,
            characters: self.characters + s.characters,
            bytes: self.bytes + s.bytes,
        }
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

impl Add for Stats {
    type Output = Stats;

    fn add(self, rhs: Self) -> Self::Output {
        self.combine(&rhs)
    }
}
