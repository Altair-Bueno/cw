use crate::stats::automata::posix_ascii::PosixASCII;
use crate::stats::automata::posix_utf8::PosixUTF8;
use crate::stats::Stats;
use std::fmt::{Display, Formatter};
use std::io::BufRead;
use std::str::FromStr;
use Encoding::*;
use LineBreak::*;

pub use automata_trait::Automata;

mod automata_trait;
pub(crate) mod partial_state;
pub mod posix_ascii;
pub mod posix_utf8;

// If we are on a word or not
type OnWord = bool;

const STR_UTF8: &str = "UTF8";
const STR_ASCII: &str = "ASCII";

#[derive(Clone)]
pub enum Encoding {
    UTF8,
    ASCII,
}
impl Display for Encoding {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let w = match self {
            UTF8 => STR_UTF8,
            ASCII => STR_ASCII,
        };
        write!(f, "{}", w)
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
            STR_UTF8 => Ok(UTF8),
            STR_ASCII => Ok(ASCII),
            _ => Err("Invalid encoding".to_string()),
        }
    }
}

const STR_CRLF: &str = "CRLF";
const STR_LF: &str = "LF";
#[derive(Clone)]
pub enum LineBreak {
    CRLF,
    LF,
}

impl Display for LineBreak {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let w = match self {
            CRLF => STR_CRLF,
            LF => STR_LF,
        };
        write!(f, "{}", w)
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
            STR_LF => Ok(LF),
            STR_CRLF => Ok(CRLF),
            _ => Err("Invalid line break type".to_string()),
        }
    }
}

#[derive(Default,Clone)]
pub struct Mode(Encoding, LineBreak);

impl Mode {
    pub fn new(encoding: Encoding, line_break: LineBreak) -> Mode {
        Mode(encoding, line_break)
    }

    pub fn proccess(&self, read: Box<dyn BufRead>) -> std::io::Result<Stats> {
        match self {
            Mode(UTF8, LF) => PosixUTF8.stats_from_bufread(read),
            Mode(ASCII, LF) => PosixASCII.stats_from_bufread(read),
            _ => todo!(),
        }
    }
}

impl Display for Mode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.0, self.1)
    }
}
