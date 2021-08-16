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

const BUFFER_SIZE: usize = 16 * 1024; // 8KB

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

        // todo encoding not used right now
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
