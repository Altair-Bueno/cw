use std::fmt::{Display, Formatter};
use std::io::{BufRead, Error};

use crate::cw_lib::parser_config::encoding::Encoding;
use crate::cw_lib::parser_config::line_break::LineBreak;
use crate::cw_lib::stats::Stats;
use clap::ArgMatches;
use crate::cw_lib::state::State;
use crate::cw_lib::state::traits::PartialState;

const BUFFER_SIZE: usize = 8 * 1024; // 8KB


pub struct Parser {
    encoding :Encoding,
    linebreak:LineBreak,
    tranformer: fn(State,&[u8]) -> State,
    stats_format: Stats,
}
impl Default for Parser {
    fn default() -> Self {
        Parser {
            encoding: Default::default(),
            linebreak: Default::default(),
            tranformer:
            |x,y|
                x
                    .words(y)
                    .lines(y)
                    .chars(y)
                    .bytes(y)
                    .max_length(y),
            stats_format: Default::default()
        }
    }
}

impl Parser {
    pub fn new(
        encoding: Encoding,
        linebreak: LineBreak,
        lines:bool,
        words:bool,
        chars:bool,
        bytes:bool,
        max_length:bool
    ) -> Parser {
        let mut tranformer: fn(State,&[u8]) -> State =|x,_| x;

        let lines = if lines {
            tranformer = |x,y| tranformer(x,y).lines(y);
            Some(0)
        } else { None };
        let words = if words {
            tranformer = |x,y| tranformer(x,y).words(y);
            Some(0)
        } else { None };
        let characters = if chars {
            tranformer = |x,y| tranformer(x,y).chars(y);
            Some(0)
        } else { None };
        let bytes = if bytes {
            tranformer = move |x,y| tranformer(x,y).bytes(y);
            Some(0)
        } else { None };
        let legth = if max_length {
            tranformer = move |x,y| tranformer(x,y).max_length(y);
            Some(0)
        } else { None };

        Parser {
            encoding,
            linebreak,
            tranformer,
            stats_format: Stats::new(
                lines,
                words,
                characters,
                bytes,
                legth,
            ),
        }
    }

    pub fn from_clap(args: &ArgMatches) -> Parser {
        todo!()
        /*let encoding = args
            .value_of("encoding")
            .map(|x| x.parse().unwrap_or_default())
            .unwrap_or_default();
        let breakk = args
            .value_of("break")
            .map(|x| x.parse().unwrap_or_default())
            .unwrap_or_default();*/
    }

    pub fn proccess<R: BufRead + Sized>(&self, mut reader: R) -> std::io::Result<Stats> {
        let mut state = State::default();
        let mut buff = [0; BUFFER_SIZE];
        loop {
            let read = reader.read(&mut buff)?;
            if read == 0 {
                let (l,w,c,b,m) = state.output();
                return Ok(Stats::new(Some(l),Some(w),Some(c),Some(b),Some(m)));
            }
            state = (self.tranformer)(state,&buff[0..read]);
        }
    }
}

#[cfg(test)]
mod test {
    use super::Parser;
    use crate::cw_lib::parser_config::encoding::Encoding;
    use crate::cw_lib::parser_config::line_break::LineBreak;

    #[test]
    pub fn test1() {
        todo!();
        let parse = Parser::new(Encoding::UTF8,LineBreak::LF,true,true,true,true,true);
    }
}

