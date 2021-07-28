use std::fmt::{Display, Formatter};
use std::io::BufRead;

use crate::cw::automaton::automaton_utf8::AutomatonUTF8;
use crate::cw::automaton::trait_automaton::Automata;
use crate::cw::parser_config::encoding::Encoding;
use crate::cw::parser_config::line_break::LineBreak;
use crate::cw::stats::Stats;
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
    pub fn proccess<R: BufRead + Sized>(&self, read: R) -> std::io::Result<Stats> {
        match self {
            Parser(Encoding::UTF8, LineBreak::LF) => AutomatonUTF8.stats_from_bufread(read, b'\n'),
            Parser(Encoding::UTF8, LineBreak::CR) => AutomatonUTF8.stats_from_bufread(read, b'\r'),
            _ => todo!(), // UTF16
        }
    }
}

impl Display for Parser {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.0, self.1)
    }
}
