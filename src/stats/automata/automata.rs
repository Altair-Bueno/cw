use std::fmt::{Display, Formatter};
use std::io::BufRead;

use Encoding::*;
use LineBreak::*;

use crate::stats::automata::ascii::posix_ascii::PosixASCII;
use crate::stats::automata::automata_trait::Automata;
use crate::stats::automata::encoding::Encoding;
use crate::stats::automata::line_break::LineBreak;
use crate::stats::automata::utf8::posix_utf8::PosixUTF8;
use crate::stats::Stats;

#[derive(Default, Clone)]
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
