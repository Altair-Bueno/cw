use crate::commandline::Cwargs;
use std::fmt::{Display, Formatter};
use std::io::{BufRead, Read};
use std::ops::Add;
use crate::stats::analizer::{PartialResponse, automata};

#[derive(Debug, Default)]
pub struct Stats {
    pub lines: u32,
    pub words: u32,
    pub characters: u32,
    pub bytes: u32,
    //colums: Colums,
}

impl Stats {
    pub fn new() -> Stats {
        Stats::default()
    }
    pub fn from_file(mut reader: Box<dyn BufRead>) -> std::io::Result<Stats> {
        // TODO not completly done
        let mut state = PartialResponse::new();
        loop {
            let mut buff = [0; 1024];
            let read = reader.read(&mut buff)?;
            if read == 0 {
                return Ok(state.result());
            }
            state = automata(state,&buff[0..read]);
        }
    }
    pub fn combine(&self, s: &Stats) -> Stats {
        Stats {
            lines: self.lines + s.lines,
            words: self.words + s.words,
            characters: self.characters + s.characters,
            bytes: self.bytes + s.bytes,
        }
    }
    pub fn show(&self, args: &Cwargs) -> String {
        let mut changes = false;
        let mut string = String::new();

        if args.allows_lines() {
            changes = true;
            string = format!("{}\t{}", string, self.lines)
        }
        if args.allows_words() {
            changes = true;
            string = format!("{}\t{}", string, self.words)
        }
        if args.allows_characters() {
            changes = true;
            string = format!("{}\t{}", string, self.characters)
        }
        if args.allows_bytes() {
            changes = true;
            string = format!("{}\t{}", string, self.bytes)
        }
        /*
        if columns {
            changes = true;
            string = format!("{},{}", string, self.colums)
        }
        */
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
