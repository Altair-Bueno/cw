use crate::stats::Stats;
use Encoding::*;
use LineBreak::*;
use crate::stats::automata::posix_utf8::PosixUTF8;
use crate::stats::automata::Automata;
use std::io::BufRead;
use std::str::FromStr;
use crate::stats::automata::posix_ascii::PosixASCII;
use std::fmt::{Display, Formatter};

const utf8 : &str = "UTF8";
const ascii : &str = "ASCII";

pub enum Encoding {
    UTF8,
    ASCII,
}
impl Display for Encoding {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let w = match self {
            UTF8 => utf8,
            ASCII => ascii,
        };
        write!(f,"{}",w)
    }
}
impl Default for Encoding {
    fn default() -> Self {
        Encoding::ASCII
    }
}

impl FromStr for Encoding {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            utf8 => Ok(UTF8),
            ascii => Ok(ASCII),
            _=> Err("Invalid encoding".to_string())
        }
    }
}

const crlf : &str = "CRLF";
const lf : &str = "LF";
pub enum LineBreak {
    CRLF,
    LF,
}

impl Display for LineBreak {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let w = match self {
            CRLF => crlf,
            LF => lf,
        };
        write!(f,"{}",w)
    }
}

impl Default for LineBreak {
    fn default() -> Self {
        LineBreak::LF
    }
}

impl FromStr for LineBreak {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            lf=> Ok(LF),
            crlf=> Ok(CRLF),
            _ => Err("Invalid line break type".to_string())
        }
    }
}

#[derive(Default)]
pub struct Mode (
    Encoding,
    LineBreak,
    );


impl Mode {
    pub fn new(encoding: Encoding, line_break: LineBreak) -> Mode {
        Mode(encoding,line_break)
    }

    pub fn proccess(&self, read : Box<dyn BufRead>) -> std::io::Result<Stats> {
        match self {
            Mode(UTF8,LF) => PosixUTF8.stats_from_bufread(read),
            Mode(ASCII,LF) => PosixASCII.stats_from_bufread(read),
            _=> todo!()
        }
    }
}

impl Display for Mode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"{} {}",self.0,self.1)
    }
}
