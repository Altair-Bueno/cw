use crate::cw_lib::state::bytes_state::BytesState;
use crate::cw_lib::state::chars_state::CharState;
use crate::cw_lib::state::lines_state::LinesState;
use crate::cw_lib::state::max_length::MaxLengthState;
use crate::cw_lib::state::traits::{compute::Compute,partial_state::PartialState};
use crate::cw_lib::state::words_state::WordsState;
use crate::Stats;
use std::fmt::{Display, Formatter};

pub mod bytes_state;
pub mod chars_state;
pub mod lines_state;
pub mod max_length;
pub mod traits;
pub mod words_state;

#[derive(Copy, Clone, Debug)]
pub struct State {
    lines_state: Option<LinesState>,
    words_state: Option<WordsState>,
    char_state: Option<CharState>,
    bytes_state: Option<BytesState>,
    max_length_state: Option<MaxLengthState>,
}
impl Default for State {
    fn default() -> Self {
        State {
            lines_state: Some(LinesState::default()),
            words_state: Some(WordsState::new()),
            char_state: None,
            bytes_state: Some(BytesState::new()),
            max_length_state: None,
        }
    }
}

impl PartialState for State {
    type Output = Stats;

    fn output(&self) -> Self::Output {
        let lines = self.lines_state.map(|x| x.output());
        let words = self.words_state.map(|x| x.output());
        let characters = self.char_state.map(|x| x.output());
        let bytes = self.bytes_state.map(|x| x.output());
        let len = self.max_length_state.map(|x| x.output());
        Stats::new(lines, words, characters, bytes, len)
    }
}
impl Compute for State {
    fn compute(self, tape: &[u8]) -> Self {
        State {
            lines_state: self.lines_state.map(|x| x.compute(tape)),
            words_state: self.words_state.map(|x| x.compute(tape)),
            char_state: self.char_state.map(|x| x.compute(tape)),
            bytes_state: self.bytes_state.map(|x| x.compute(tape)),
            max_length_state: self.max_length_state.map(|x| x.compute(tape)),
        }
    }
}

// fn(State,&[u8]) -> State
impl State {
    pub fn new() -> State {
        State {
            lines_state: None,
            words_state: None,
            char_state: None,
            bytes_state: None,
            max_length_state: None,
        }
    }

    pub fn set_lines_state(&mut self, lines_state: Option<LinesState>) {
        self.lines_state = lines_state;
    }
    pub fn set_words_state(&mut self, words_state: Option<WordsState>) {
        self.words_state = words_state;
    }
    pub fn set_char_state(&mut self, char_state: Option<CharState>) {
        self.char_state = char_state;
    }
    pub fn set_bytes_state(&mut self, bytes_state: Option<BytesState>) {
        self.bytes_state = bytes_state;
    }
    pub fn set_max_length_state(&mut self, max_length_state: Option<MaxLengthState>) {
        self.max_length_state = max_length_state;
    }
}

impl Iterator for State {
    type Item = State;

    fn next(&mut self) -> Option<Self::Item> {
        Some(*self)
    }
}
impl Display for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(x) = self.lines_state {
            write!(f, "l({})\t",x.linebreak())?;
        }
        if let Some(_) = self.words_state {
            write!(f, "w\t")?;
        }
        if let Some(_x) = self.char_state {
            // TODO encoding
            write!(f, "c\t")?;
        }
        if let Some(_) = self.bytes_state {
            write!(f, "b\t")?;
        }
        if let Some(_x) = self.max_length_state {
            write!(f, "L\t")?;
        }

        Ok(())
    }
}
