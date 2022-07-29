use std::fmt::{Display, Formatter};

use crate::state::bytes::BytesState;
use crate::state::chars::CharState;
use crate::state::lines::LinesState;
use crate::state::max_length::MaxLengthState;
use crate::state::words::WordsState;
use crate::Stats;
use crate::traits::{compute::Compute, partial_state::PartialState};

pub mod bytes;
pub mod chars;
pub mod lines;
pub mod max_length;
pub mod words;

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

    fn output(self) -> Self::Output {
        let lines = self.lines_state.map(|x| x.output());
        let words = self.words_state.map(|x| x.output());
        let characters = self.char_state.map(|x| x.output());
        let bytes = self.bytes_state.map(|x| x.output());
        let len = self.max_length_state.map(|x| x.output());
        Stats::new(lines, words, characters, bytes, len)
    }
}

impl Compute for State {
    fn utf8_compute(self, tape: &[u8]) -> Self {
        State {
            lines_state: self.lines_state.map(|x| x.utf8_compute(tape)),
            words_state: self.words_state.map(|x| x.utf8_compute(tape)),
            char_state: self.char_state.map(|x| x.utf8_compute(tape)),
            bytes_state: self.bytes_state.map(|x| x.utf8_compute(tape)),
            max_length_state: self.max_length_state.map(|x| x.utf8_compute(tape)),
        }
    }

    fn utf16_compute(self, tape: &[u8]) -> Self {
        State {
            lines_state: self.lines_state.map(|x| x.utf16_compute(tape)),
            words_state: self.words_state.map(|x| x.utf16_compute(tape)),
            char_state: self.char_state.map(|x| x.utf16_compute(tape)),
            bytes_state: self.bytes_state.map(|x| x.utf16_compute(tape)),
            max_length_state: self.max_length_state.map(|x| x.utf16_compute(tape)),
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
        self.lines_state
            .map(|_| write!(f, "l\t"))
            .unwrap_or(Ok(()))?;
        self.words_state
            .map(|_| write!(f, "w\t"))
            .unwrap_or(Ok(()))?;
        self.char_state
            .map(|_| write!(f, "c\t"))
            .unwrap_or(Ok(()))?;
        self.bytes_state
            .map(|_| write!(f, "b\t"))
            .unwrap_or(Ok(()))?;
        self.max_length_state
            .map(|_| write!(f, "L\t"))
            .unwrap_or(Ok(()))?;

        /*
        if let Some(x) = self.lines_state {
            write!(f, "l\t")?;
        }
        if let Some(_x) = self.words_state {
            write!(f, "w\t")?;
        }
        if let Some(_x) = self.char_state {

            write!(f, "c\t")?;
        }
        if let Some(_x) = self.bytes_state {
            write!(f, "b\t")?;
        }
        if let Some(_x) = self.max_length_state {
            write!(f, "L\t")?;
        }
        */
        Ok(())
    }
}
