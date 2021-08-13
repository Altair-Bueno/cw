use std::cmp::max;
use std::fmt::{Display, Formatter};
use std::option::Option::Some;

/// Represents Stats for a file
#[derive(Debug, Default, Eq, PartialEq)]
pub struct Stats {
    pub lines: Option<u32>,
    pub words: Option<u32>,
    pub characters: Option<u32>,
    pub bytes: Option<u32>,
    pub legth: Option<u32>,
    //colums: Colums,
}

impl Stats {
    /// Creates a new Stats struct using the given parameters
    pub fn new(lines: Option<u32>, words: Option<u32>, characters: Option<u32>, bytes: Option<u32>, legth: Option<u32>) -> Stats {
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
        let combine_using =  |a, b, f:fn(u32,u32)->u32| {
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