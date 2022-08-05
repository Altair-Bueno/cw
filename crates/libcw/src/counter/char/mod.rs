#[cfg(feature = "tower")]
mod service;
#[cfg(feature = "tower")]
pub use service::*;

use super::Counter;
use bytecount::num_chars;

#[derive(Debug, Default, Clone)]
pub struct CharCounterState {
    count: usize,
}

#[derive(Debug, Default, Clone)]
pub struct CharCounter;

impl CharCounter {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Counter for CharCounter {
    type State = CharCounterState;
    type Output = usize;

    fn parse(&self, input: &[u8], mut state: Self::State) -> Self::State {
        state.count += num_chars(input);
        state
    }

    fn terminate(&self, state: Self::State) -> usize {
        state.count
    }
}
