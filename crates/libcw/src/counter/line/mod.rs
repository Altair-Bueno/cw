#[cfg(feature="tower")]
mod service;
#[cfg(feature="tower")]
pub use service::*;

use crate::config::LineBreak;

use super::Counter;
use bytecount::count;

#[derive(Debug, Default, Clone)]
pub struct LineCounterState {
    count: usize,
}

#[derive(Debug, Default, Clone)]
pub struct LineCounter {
    linebreak: LineBreak,
}

impl LineCounter {
    pub fn new(linebreak: LineBreak) -> Self {
        Self { linebreak }
    }
}

impl Counter<&[u8]> for LineCounter {
    type State = LineCounterState;
    type Output = usize;

    fn parse(&self, input: &[u8], mut state: Self::State) -> Self::State {
        state.count += count(input, self.linebreak.into());
        state
    }

    fn terminate(&self, state: Self::State) -> usize {
        state.count
    }
}
