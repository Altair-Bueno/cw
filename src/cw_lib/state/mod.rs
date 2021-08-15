use crate::cw_lib::state::lines_state::LinesState;
use crate::cw_lib::state::words_state::WordsState;
use crate::cw_lib::state::bytes_state::BytesState;
use crate::cw_lib::state::max_length::MaxLengthState;
use crate::cw_lib::state::chars_state::CharState;
use crate::cw_lib::state::traits::{PartialState, Compute};

pub mod traits;
pub mod lines_state;
pub mod words_state;
pub mod bytes_state;
pub mod max_length;
mod chars_state;

#[derive(Default,Copy, Clone)]
pub struct State {
    lines_state: LinesState,
    words_state: WordsState,
    char_state: CharState,
    bytes_state: BytesState,
    max_length_state: MaxLengthState,
}

impl PartialState for State {
    type Output = (usize,usize,usize,usize,usize);

    fn output(&self) -> Self::Output {
        let lines_state = self.lines_state.output();
        let words_state = self.words_state.output();
        let chars_state = self.char_state.output();
        let bytes_state = self.bytes_state.output();
        let max_length = self.max_length_state.output();

        (lines_state,words_state,chars_state,bytes_state,max_length)
    }
}

// fn(State,&[u8]) -> State
impl State {
    pub fn new(linebreak:u8) -> State {
        State{
            lines_state: LinesState::new(linebreak),
            words_state: WordsState::new(),
            char_state: CharState::new(),
            bytes_state: BytesState::new(),
            max_length_state: MaxLengthState::new(linebreak)
        }
    }
    pub fn none(self, _:&[u8]) -> State {
        self
    }
    pub fn lines(mut self, tape : &[u8]) ->State {
        self.lines_state = self.lines_state.compute(tape);
        self
    }
    pub fn words(mut self, tape:&[u8]) -> State {
        self.words_state = self.words_state.compute(tape);
        self
    }
    pub fn chars(mut self, tape:&[u8]) -> State {
        self.char_state = self.char_state.compute(tape);
        self
    }
    pub fn bytes(mut self, tape:&[u8]) -> State {
        self.bytes_state = self.bytes_state.compute(tape);
        self
    }
    pub fn max_length(mut self, tape:&[u8]) -> State {
        self.max_length_state = self.max_length_state.compute(tape);
        self
    }
}

impl Iterator for State {
    type Item = State;

    fn next(&mut self) -> Option<Self::Item> {
        Some(*self)
    }
}