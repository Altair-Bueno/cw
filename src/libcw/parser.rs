use std::fmt::{Display, Formatter};
use std::io::BufRead;

use crate::libcw::config::Encoding;
use crate::libcw::config::LineBreak;
use crate::libcw::state::bytes_state::BytesState;
use crate::libcw::state::chars_state::CharState;
use crate::libcw::state::lines_state::LinesState;
use crate::libcw::state::max_length::MaxLengthState;
use crate::libcw::state::State;
use crate::libcw::state::traits::{compute::Compute, partial_state::PartialState};
use crate::libcw::state::words_state::WordsState;
use crate::libcw::stats::Stats;

const BUFFER_SIZE: usize = 16 * 1024; // 8KB

#[derive(Default, Copy, Clone, Debug)]
pub struct Parser {
    initial_state: State,
}

impl Parser {
    pub fn new(
        encoding: Encoding,
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
            initial_state.set_lines_state(Some(LinesState::new(linebreak)))
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
            initial_state.set_max_length_state(Some(MaxLengthState::new(linebreak, encoding)))
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
impl Display for Parser {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.initial_state.fmt(f)
    }
}
