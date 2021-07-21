use std::fmt::{Display, Formatter};
use std::io::{BufReader, BufRead};
use crate::commandline::Arguments;
use std::ops::Add;

#[derive(Debug,Default)]
pub struct Stats {
    lines: u32,
    words: u32,
    characters: u32,
    bytes: u32,
    //colums: Colums,
}

impl Stats {
    pub fn new() -> Stats {
        Stats::default()
    }
    pub fn from_file(reader : Box<dyn BufRead>) -> std::io::Result<Stats> {
        // TODO not completly done
        let stats = reader
            .lines()
            .map(|x| x.unwrap())
            .fold(Stats::new(), |mut stats,new| {
                stats.lines = stats.lines + 1;
                stats
            });

        Ok(stats)
    }
    pub fn combine (&self , s:&Stats) -> Stats {
        Stats {
            lines: self.lines + s.lines,
            words: self.words + s.words,
            characters: self.characters + s.characters,
            bytes: self.bytes + s.bytes,
        }
    }
    pub fn show(&self, args : &Arguments) -> String {
        let mut changes = false;
        let mut string = String::new();

        if lines {
            changes = true;
            string = format!("{}\t{}", string, self.lines)
        }
        if words {
            changes = true;
            string = format!("{}\t{}", string, self.words)
        }
        if characters {
            changes = true;
            string = format!("{}\t{}", string, self.characters)
        }
        if bytes {
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
            format!("{}",self)
        } else {
            string
        }
    }
}
impl Display for Stats {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,"{}\t{}\t{}\t{}",
            self.lines,
            self.words,
            self.characters,
            self.bytes
        )
    }
}

impl Add for Stats {
    type Output = Stats;

    fn add(self, rhs: Self) -> Self::Output {
        self.combine(&rhs)
    }
}
