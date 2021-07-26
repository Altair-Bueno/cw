use std::fmt::{Display, Formatter};
use std::io::BufRead;

use Encoding::*;
use LineBreak::*;

use crate::stats::automata::ascii::posix_ascii::PosixASCII;
use crate::stats::automata::trait_automata::Automata;
use crate::stats::automata::encoding::Encoding;
use crate::stats::automata::line_break::LineBreak;
use crate::stats::automata::utf8::posix_utf8::PosixUTF8;
use crate::stats::Stats;
use clap::ArgMatches;

#[derive(Default, Clone)]
pub struct AutomataConfig(Encoding, LineBreak);

impl AutomataConfig {
    pub fn new(encoding: Encoding, line_break: LineBreak) -> AutomataConfig {
        AutomataConfig(encoding, line_break)
    }

    pub fn from_clap(args: &ArgMatches) -> AutomataConfig {
        let encoding = args
            .value_of("encoding")
            .map(|x| x.parse().unwrap_or_default())
            .unwrap_or_default();
        let breakk = args
            .value_of("break")
            .map(|x| x.parse().unwrap_or_default())
            .unwrap_or_default();
        AutomataConfig(encoding, breakk)
    }

    pub fn proccess(&self, read: Box<dyn BufRead>) -> std::io::Result<Stats> {
        match self {
            AutomataConfig(UTF8, LF) => PosixUTF8.stats_from_bufread(read),
            AutomataConfig(ASCII, LF) => PosixASCII.stats_from_bufread(read),
            _ => todo!(),
        }
    }
}

impl Display for AutomataConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.0, self.1)
    }
}
