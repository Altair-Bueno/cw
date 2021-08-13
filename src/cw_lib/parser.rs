use std::fmt::{Display, Formatter};
use std::io::BufRead;

use crate::cw_lib::parser_config::encoding::Encoding;
use crate::cw_lib::parser_config::line_break::LineBreak;
use crate::cw_lib::stats::Stats;
use clap::ArgMatches;
use crate::cw_lib::func::helpers::State;

const BUFFER_SIZE: usize = 8 * 1024; // 8KB


#[derive(Clone)]
pub struct Parser {
    encoding :Encoding,
    linebreak:LineBreak,
    tranformer: fn(State,&[u8]) -> State,
    default_state : State
}
impl Default for Parser {
    fn default() -> Self {
        Parser {
            encoding: Default::default(),
            linebreak: Default::default(),
            tranformer: todo!(),
            default_state: Default::default(),
        }
    }
}

impl Parser {
    pub fn new(
        encoding: Encoding,
        line_break: LineBreak,
        lines:bool,
        words:bool,
        chars:bool,
        bytes:bool,
        max_length:bool
    ) -> Parser {
        Parser {
            encoding,
            linebreak: Default::default(),
            tranformer: (),
            default_state: (),
        }
    }

    pub fn from_clap(args: &ArgMatches) -> Parser {
        todo!();
        let encoding = args
            .value_of("encoding")
            .map(|x| x.parse().unwrap_or_default())
            .unwrap_or_default();
        let breakk = args
            .value_of("break")
            .map(|x| x.parse().unwrap_or_default())
            .unwrap_or_default();
    }

    pub fn proccess<R: BufRead + Sized>(&self, mut reader: R) -> std::io::Result<Stats> {
        let state = *self.default_state;
        let mut buff = [0; BUFFER_SIZE];
        loop {
            let read = reader.read(&mut buff)?;
            if read == 0 {
                return Ok(state.result());
            }
            state = self.tranformer(state,&buff[0..read]);
        }
    }
}

impl Display for Parser {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.0, self.1)
    }
}
