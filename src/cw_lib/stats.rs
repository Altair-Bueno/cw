use std::cmp::max;
use std::fmt::{Display, Formatter};
use std::option::Option::Some;

/// Represents Stats for a file
#[derive(Debug, Eq, PartialEq,Copy, Clone)]
pub struct Stats {
    lines: usize,
    words: usize,
    characters: usize,
    bytes: usize,
    legth: usize,
    print:[bool;5]
    //colums: Colums,
}

impl Default for Stats {
    fn default() -> Self {
        Stats{
            lines: 0,
            words: 0,
            characters: 0,
            bytes: 0,
            legth: 0,
            print: [true;5]
        }
    }
}

impl Stats {
    pub fn new () -> Stats {
        Stats{
            lines: 0,
            words: 0,
            characters: 0,
            bytes: 0,
            legth: 0,
            print: Default::default()
        }
    }


    pub fn print_lines(mut self, s:bool) ->Self {
        self.print[0] = s;
        self
    }
    pub fn print_words(mut self, s:bool) ->Self {
        self.print[1] = s;
        self
    }
    pub fn print_characters(mut self, s:bool) ->Self {
        self.print[2] = s;
        self
    }
    pub fn print_bytes(mut self, s:bool) ->Self {
        self.print[3] = s;
        self
    }
    pub fn print_max_lenght(mut self, s:bool) ->Self {
        self.print[4] = s;
        self
    }

    /// Combines two stats. Usefull when buffering a file. Consumes both
    /// arguments for improved performance. There is no need to
    /// de-referenciate or alloc more memory
    pub fn combine(self, s: Stats) -> Stats {
        Stats {
            lines: self.lines + s.lines,
            words: self.words + s.words,
            characters: self.characters + s.characters,
            bytes: self.bytes + s.bytes,
            legth: max(self.legth,s.legth),
            ..self
        }
    }
    pub fn set_lines(&mut self, lines: usize) {
        self.lines = lines;
    }
    pub fn set_words(&mut self, words: usize) {
        self.words = words;
    }
    pub fn set_characters(&mut self, characters: usize) {
        self.characters = characters;
    }
    pub fn set_bytes(&mut self, bytes: usize) {
        self.bytes = bytes;
    }
    pub fn set_legth(&mut self, legth: usize) {
        self.legth = legth;
    }
    pub fn set_print(&mut self, print: [bool; 5]) {
        self.print = print;
    }
}

impl Display for Stats {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut dirty = false;
        if self.print[0] {
            dirty = true;
            write!(f,"{}\t",self.lines)?;
        }
        if self.print[1] {
            dirty = true;
            write!(f,"{}\t",self.words)?;
        }
        if self.print[2] {
            dirty = true;
            write!(f,"{}\t",self.characters)?;
        }
        if self.print[3] {
            dirty = true;
            write!(f,"{}\t",self.bytes)?;
        }
        if self.print[4] {
            dirty = true;
            write!(f,"{}\t",self.legth)?;
        }
        Ok(())
    }
}