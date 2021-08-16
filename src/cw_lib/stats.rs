use std::cmp::max;
use std::fmt::{Display, Formatter};
use std::option::Option::Some;

/// Represents Stats for a file
#[derive(Debug, Eq, PartialEq)]
pub struct Stats {
    lines: Option<usize>,
    words: Option<usize>,
    characters: Option<usize>,
    bytes: Option<usize>,
    legth: Option<usize>,
    //colums: Colums,
}
impl Default for Stats {
    fn default() -> Self {
        Stats {
            lines: Some(0),
            words: Some(0),
            characters: Some(0),
            bytes: Some(0),
            legth: Some(0)
        }
    }
}

impl Stats {
    /// Creates a new Stats struct using the given parameters
    pub fn new(lines: Option<usize>, words: Option<usize>, characters: Option<usize>, bytes: Option<usize>, legth: Option<usize>) -> Stats {
        Stats {
            lines,
            words,
            characters,
            bytes,
            legth,
        }
    }

    /// Combines two stats. Usefull when buffering a file. Consumes both
    /// arguments for improved performance. There is no need to
    /// de-referenciate or alloc more memory
    pub fn combine(self, s: Stats) -> Stats {
        let combine_using =  |a, b, f:fn(usize,usize)->usize| {
            match (a,b) {
                (Some(x),Some(y)) => Some(f(x,y)),
                _ => None
            }
        };

        Stats {
            lines: combine_using(self.lines, s.lines, std::ops::Add::add),
            words: combine_using(self.words, s.words, std::ops::Add::add),
            characters: combine_using(self.characters, s.characters, std::ops::Add::add),
            bytes: combine_using(self.bytes, s.bytes, std::ops::Add::add),
            legth: combine_using(self.legth, s.legth, max),
        }
    }

    pub fn lines(&self) -> Option<usize> {
        self.lines
    }
    pub fn words(&self) -> Option<usize> {
        self.words
    }
    pub fn characters(&self) -> Option<usize> {
        self.characters
    }
    pub fn bytes(&self) -> Option<usize> {
        self.bytes
    }
    pub fn legth(&self) -> Option<usize> {
        self.legth
    }
}

impl Display for Stats {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(x) = self.lines {
            write!(f,"{}\t",x)?;
        }
        if let Some(x) = self.words {
            write!(f,"{}\t",x)?;
        }
        if let Some(x) = self.characters {
            write!(f,"{}\t",x)?;
        }
        if let Some(x) = self.bytes {
            write!(f,"{}\t",x)?;
        }
        if let Some(x) = self.legth {
            write!(f,"{}\t",x)?;
        }

        Ok(())
    }
}