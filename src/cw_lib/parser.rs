use std::io::{BufRead, Error};

use crate::cw_lib::parser_config::encoding::Encoding;
use crate::cw_lib::parser_config::line_break::LineBreak;
use crate::cw_lib::stats::Stats;
use clap::ArgMatches;
use crate::cw_lib::state::State;
use crate::cw_lib::state::traits::PartialState;

const BUFFER_SIZE: usize = 8 * 1024; // 8KB


pub struct Parser {
    tranformer: Vec<Box<dyn Fn(State, &[u8]) -> State>>,
    stats_format: Stats,
    initial_state : State,
}
impl Default for Parser {
    fn default() -> Self {
        Parser {
            tranformer:
            vec![
                Box::new(State::lines),
                Box::new(State::chars),
                Box::new(State::bytes),
                Box::new(State::max_length),
                Box::new(State::words)
            ],
            stats_format: Default::default(),
            initial_state: Default::default()
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
        let mut tranformer :Vec<Box<dyn Fn(State, &[u8]) -> State>> = Vec::with_capacity(5);
        let initial_state = State::new(linebreak.get_separator());

        let lines = if lines {
            tranformer.push(Box::new(State::lines));
            Some(0)
        } else { None };

        let words = if words {
            tranformer.push(Box::new(State::words));
            Some(0)
        } else { None };

        let characters = if chars {
            tranformer.push(Box::new(State::chars));
            Some(0)
        } else { None };

        let bytes = if bytes {
            tranformer.push(Box::new(State::bytes));
            Some(0)
        } else { None };

        let legth = if max_length {
            tranformer.push(Box::new(State::max_length));
            Some(0)
        } else { None };

        Parser {
            tranformer,
            stats_format: Stats::new(
                lines,
                words,
                characters,
                bytes,
                legth,
            ),
            initial_state
        }
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
        let lines = args.is_present("lines");
        let words = args.is_present("words");
        let characters = args.is_present("characters");
        let bytes = args.is_present("bytes");
        let len = args.is_present("line_length");

        if lines == words && lines == characters && lines == bytes && lines == len {
            Parser::new(encoding,breakk,lines,words,characters,bytes,len)
        } else {
            Parser::default()
        }
    }

    pub fn proccess<R: BufRead + Sized>(&self, mut reader: R) -> std::io::Result<Stats> {
        let mut state = self.initial_state;
        let mut buff = [0; BUFFER_SIZE];
        loop {
            let read = reader.read(&mut buff)?;
            if read == 0 {
                let (l,w,c,b,m) = state.output();
                return Ok(Stats::new(Some(l),Some(w),Some(c),Some(b),Some(m)));
            }
            state = self.tranformer.iter().fold(state,|acc,n| n(acc,&buff[0..read]));
        }
    }
}

#[cfg(test)]
mod test {
    use super::Parser;
    use crate::cw_lib::parser_config::encoding::Encoding;
    use crate::cw_lib::parser_config::line_break::LineBreak;
    use crate::cw_lib::stats::Stats;
    use std::fs::File;
    use std::io::BufReader;

    fn proccess_file_test(f: &str) -> Stats {
        let reader = BufReader::new(File::open(f).unwrap());
        let parser = Parser::new(Encoding::UTF8,LineBreak::LF,true,true,true,true,true);
        parser.proccess(reader).unwrap()
    }

    #[test]
    fn gabriel() {
        let out = proccess_file_test("tests/resources/Gabriel.txt");
        let expected = Stats::new(Some(57), Some(187), Some(2694), Some(2700), Some(580));
        assert_eq!(out, expected)
    }

    #[test]
    fn lorem() {
        let out = proccess_file_test("tests/resources/Lorem_big.txt");
        let expected = Stats::new(Some(1996), Some(111618), Some(751539), Some(751539), Some(1142));
        assert_eq!(out, expected)
    }
    #[test]
    #[ignore] // On CI does fail. I don't know why
    fn bible() {
        let out = proccess_file_test("tests/resources/bible.txt");
        let expected = Stats::new(Some(100182), Some(824036), Some(4451368), Some(4451368), Some(78));
        assert_eq!(out, expected)
    }
    #[test]
    fn s1() {
        let out = proccess_file_test("tests/resources/sample1.txt");
        let expected = Stats::new(Some(3), Some(88), Some(607), Some(607), Some(346));
        assert_eq!(out, expected)
    }

    #[test]
    fn s2() {
        let out = proccess_file_test("tests/resources/sample2.txt");
        let expected = Stats::new(Some(12), Some(423), Some(2859), Some(2859), Some(635));
        assert_eq!(out, expected)
    }
    #[test]
    fn s3() {
        let out = proccess_file_test("tests/resources/sample3.txt");
        let expected = Stats::new(Some(20), Some(546), Some(3541), Some(3541), Some(818));
        assert_eq!(out, expected)
    }
    #[test]
    fn small() {
        let out = proccess_file_test("tests/resources/small.txt");
        let expected = Stats::new(Some(1), Some(3), Some(18), Some(18), Some(17));
        assert_eq!(out, expected)
    }
    #[test]
    fn empty() {
        let out = proccess_file_test("tests/resources/empty.txt");
        let expected = Stats::new(Some(0), Some(0), Some(0), Some(0), Some(0));
        assert_eq!(out, expected)
    }

    #[test]
    #[ignore]
    fn arabic() {
        // todo
        // - Legth isn't 0
        // - test weird
        let out = proccess_file_test("tests/resources/arabic.txt");
        let expected = Stats::new(Some(0), Some(10), Some(58), Some(105), Some(0));
        assert_eq!(out, expected)
    }
    #[test]
    fn spanish() {
        let out = proccess_file_test("tests/resources/spanish.txt");
        let expected = Stats::new(Some(1), Some(3), Some(19), Some(22), Some(18));
        assert_eq!(out, expected)
    }

    #[test]
    fn french() {
        let out = proccess_file_test("tests/resources/french.txt");
        let expected = Stats::new(Some(0), Some(10), Some(58), Some(61), Some(58));
        assert_eq!(out, expected)
    }
}

