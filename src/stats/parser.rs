use std::fmt::{Display, Formatter};
use std::io::BufRead;

use Encoding::*;
use LineBreak::*;

use crate::stats::automata::automata_utf8::AutomataUTF8;
use crate::stats::parser_config::encoding::Encoding;
use crate::stats::parser_config::line_break::LineBreak;
use crate::stats::automata::posix_ascii::PosixASCII;
use crate::stats::automata::trait_automata::Automata;
use crate::stats::stats::Stats;
use clap::ArgMatches;

#[derive(Default, Clone)]
pub struct Parser(Encoding, LineBreak);

impl Parser {
    pub fn new(encoding: Encoding, line_break: LineBreak) -> Parser {
        Parser(encoding, line_break)
    }

    pub fn from_clap(args: &ArgMatches) -> Parser {
        let encoding = args
            .value_of("encoding")
            .map(|x| x.parse().unwrap_or_default())
            .unwrap_or_default();
        let breakk = args
            .value_of("break")
            .map(|x| x.parse().unwrap_or_default())
            .unwrap_or_default();
        Parser(encoding, breakk)
    }
    pub fn proccess(&self, read: Box<dyn BufRead>) -> std::io::Result<Stats> {
        match self {
            Parser(UTF8, LF) => AutomataUTF8.stats_from_bufread(read),
            Parser(ASCII, LF) => PosixASCII.stats_from_bufread(read),
            _ => todo!(),
        }
    }
}

impl Display for Parser {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.0, self.1)
    }
}
