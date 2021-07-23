use crate::commandline::Cwargs;
use crate::stats::automata::PartialResponse;
use crate::stats::Automata;
use std::fmt::{Display, Formatter};
use std::io::{BufRead, Read};
use std::ops::Add;

const BUFFER_SIZE: usize = 16 * 1024; // 16KB

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
    /// Calculates stats for a file
    pub fn from_file(mut reader: Box<dyn BufRead>) -> std::io::Result<Stats> {
        let mut state = PartialResponse::initial_state();
        // TODO use a single buffer for all operations instead
        let mut buff = [0; BUFFER_SIZE];
        loop {
            let read = reader.read(&mut buff)?;
            if read == 0 {
                return Ok(state.result());
            }
            state = Automata::run(state, &buff[0..read]);
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
    /// Returns a String representation of this struct, but only includes the
    /// requested information
    pub fn pretty_print(&self, args: &Cwargs) -> String {
        let mut changes = false;
        let mut string = String::new();

        if args.lines {
            changes = true;
            string = format!("{}\t{}", string, self.lines)
        }
        if args.words {
            changes = true;
            string = format!("{}\t{}", string, self.words)
        }
        if args.characters {
            changes = true;
            string = format!("{}\t{}", string, self.characters)
        }
        if args.bytes {
            changes = true;
            string = format!("{}\t{}", string, self.bytes)
        }
        if !changes {
            format!("{}", self)
        } else {
            string
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
