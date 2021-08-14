use std::fmt::{Display, Formatter};
use std::io::{BufRead, Error};

use crate::cw_lib::parser_config::encoding::Encoding;
use crate::cw_lib::parser_config::line_break::LineBreak;
use crate::cw_lib::stats::Stats;
use clap::ArgMatches;
use crate::cw_lib::state::State;
use crate::cw_lib::state::traits::PartialState;

const BUFFER_SIZE: usize = 8 * 1024; // 8KB

mod compose {
    #[macro_export]
    macro_rules! compose {
    ( $last:expr ) => { $last };
    ( $head:expr, $($tail:expr), +) => {
        compose_two($head, compose!($($tail),+))
    };
    }

    pub fn compose_two<A, B, C, G, F>(f: F, g: G) -> impl Fn(A) -> C
        where
            F: Fn(A) -> B,
            G: Fn(B) -> C,
    {
        move |x| g(f(x))
    }
}

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
        linebreak: LineBreak,
        lines:bool,
        words:bool,
        chars:bool,
        bytes:bool,
        max_length:bool
    ) -> Parser {
        let mut tranformer: fn(State,&[u8]) -> State =|x,_| x;
/*
        if lines {
            transformer = compose::compose_two(State::lines,transformer);
        }
        if words {
            tranformer = compose::compose_two(State::words,tranformer);
        }
        if chars {
            tranformer = compose::compose_two(State::chars,tranformer);
        }
        if bytes {
            tranformer = compose::compose_two(State::bytes,tranformer);
        }
        if max_length {
            tranformer = compose::compose_two(State::max_length,tranformer);
        }
*/
        Parser {
            encoding,
            linebreak,
            tranformer,
            default_state: Default::default(),
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
        let mut state = self.default_state.clone();
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

