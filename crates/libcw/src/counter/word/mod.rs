#[cfg(feature="tower")]
mod service;
#[cfg(feature="tower")]
pub use service::*;

use super::Counter;
use std::fmt::Debug;

#[derive(Debug, Default, Clone)]
enum Location {
    #[default]
    WhiteSpace,
    Character,
}

#[derive(Debug, Default, Clone)]
pub struct WordCounterState {
    location: Location,
    count: usize,
}

#[derive(Clone)]
pub struct WordCounter {
    reductor: fn(WordCounterState, & u8) -> WordCounterState,
}

impl Debug for WordCounter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let reductor = "<FUNCTION>";
        f.debug_struct("WordCounter")
            .field("reductor", &reductor)
            .finish()
    }
}

impl Default for WordCounter {
    fn default() -> Self {
        Self::new()
    }
}


impl WordCounter {
    pub fn new() -> Self {
        let reductor = collapse_utf8;
        Self { reductor }
    }
}

impl Counter<&[u8]> for WordCounter {
    type State = WordCounterState;
    type Output = usize;

    fn parse(&self, input: &[u8], state: Self::State) -> Self::State {
        input.iter().fold(state, self.reductor)
    }

    fn terminate(&self, state: Self::State) -> usize {
        state.count
    }
}


fn collapse_utf8(state: WordCounterState, char: &u8) -> WordCounterState {
    let is_separator = match char {
        0x20 | 0x09 => true,
        char => (0x0A..=0x0D).contains(char),
    };

    let WordCounterState { location, count } = state;

    let (location, count) = match location {
        Location::WhiteSpace if is_separator => (Location::WhiteSpace, count),
        Location::Character if is_separator => (Location::WhiteSpace, count + 1),
        Location::WhiteSpace => (Location::Character, count),
        Location::Character => (Location::Character, count),
    };

    WordCounterState { location, count }
}
