use std::fmt::{Display, Formatter};
use std::io::{BufRead, Error};

use crate::cw_lib::parser_config::encoding::Encoding;
use crate::cw_lib::parser_config::line_break::LineBreak;
use crate::cw_lib::stats::Stats;
use clap::ArgMatches;
use crate::cw_lib::state::State;
use crate::cw_lib::state::traits::PartialState;

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
        linebreak: LineBreak,
        lines:bool,
        words:bool,
        chars:bool,
        bytes:bool,
        max_length:bool
    ) -> Parser {
        Parser {
            encoding,
            linebreak,
            tranformer: todo!(),
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

    pub fn proccess<R: BufRead + Sized>(&self, mut reader: R) -> Result<Stats,String> {
        let mut state = self.default_state.clone();
        let mut buff = [0; BUFFER_SIZE];
        loop {
            let read = reader.read(&mut buff);
            let read = match read {
                Ok(n) => n,
                Err(e) => return Err(e.to_string())
            };
            if read == 0 {
                return state.output();
            }
            state = (self.tranformer)(state,&buff[0..read]);
        }
    }
}

