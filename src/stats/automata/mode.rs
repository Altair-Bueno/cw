use crate::stats::Stats;
use Encoding::*;
use LineBreak::*;
use crate::stats::automata::posix_utf8::PosixUTF8;
use crate::stats::automata::Automata;
use std::io::BufRead;
use std::str::FromStr;

pub enum Encoding {
    UTF8,
    ASCII,
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
            "UTF8" => Ok(UTF8),
            "ASCII" => Ok(ASCII),
            _=> Err("Invalid encoding".to_string())
        }
    }
}
pub enum LineBreak {
    CRLF,
    LF,
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
            "LF"=> Ok(LF),
            "CRLF"=> Ok(CRLF),
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
            _=> todo!()
        }
    }
}
