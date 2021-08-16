use std::io::BufRead;

use crate::cw_lib::parser_config::encoding::Encoding;
use crate::cw_lib::parser_config::line_break::LineBreak;
use crate::cw_lib::state::bytes_state::BytesState;
use crate::cw_lib::state::chars_state::CharState;
use crate::cw_lib::state::lines_state::LinesState;
use crate::cw_lib::state::max_length::MaxLengthState;
use crate::cw_lib::state::traits::{Compute, PartialState};
use crate::cw_lib::state::words_state::WordsState;
use crate::cw_lib::state::State;
use crate::cw_lib::stats::Stats;

const BUFFER_SIZE: usize = 8 * 1024; // 8KB

#[derive(Default, Copy, Clone, Debug)]
pub struct Parser {
    initial_state: State,
}

impl Parser {
    pub fn new(
        _encoding: Encoding,
        linebreak: LineBreak,
        lines: bool,
        words: bool,
        chars: bool,
        bytes: bool,
        max_length: bool,
    ) -> Parser {
        let mut initial_state = State::new();

        // todo encoding
        // todo enable or disable searching for certain things
        if lines {
            initial_state.set_lines_state(Some(LinesState::new(linebreak.get_separator())))
        };

        if words {
            initial_state.set_words_state(Some(WordsState::new()))
        };

        if chars {
            initial_state.set_char_state(Some(CharState::new()))
        };

        if bytes {
            initial_state.set_bytes_state(Some(BytesState::new()))
        };

        if max_length {
            initial_state.set_max_length_state(Some(MaxLengthState::new(linebreak.get_separator())))
        };

        Parser { initial_state }
    }

    pub fn proccess<R: BufRead + Sized>(&self, mut reader: R) -> std::io::Result<Stats> {
        let mut state = self.initial_state;
        let mut buff = [0; BUFFER_SIZE];

        loop {
            let read = reader.read(&mut buff)?;
            if read == 0 {
                return Ok(state.output());
            }
            state = state.compute(&buff[0..read]);
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
        let parser = Parser::new(Encoding::UTF8, LineBreak::LF, true, true, true, true, true);
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
        let expected = Stats::new(
            Some(1996),
            Some(111618),
            Some(751539),
            Some(751539),
            Some(1142),
        );
        assert_eq!(out, expected)
    }
    #[test]
    #[ignore] // On CI does fail. I don't know why
    fn bible() {
        let out = proccess_file_test("tests/resources/bible.txt");
        let expected = Stats::new(
            Some(100182),
            Some(824036),
            Some(4451368),
            Some(4451368),
            Some(78),
        );
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
