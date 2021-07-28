use crate::cw_lib::automaton::trait_partial_state::PartialState;
use crate::cw_lib::stats::Stats;
use std::io::BufRead;

const BUFFER_SIZE: usize = 16 * 1024; // 16KB

/// A deterministic finite automaton that transtions using the given bufread as
/// input. It will parse each byte individually and produce stats
pub trait Automata {
    type State: PartialState + Sized;

    fn run(&self, partial: Self::State, tape: &[u8], linebreak: u8) -> Self::State;

    /// Produces stats the given reader.
    fn stats_from_bufread<R: BufRead + Sized>(
        &self,
        mut reader: R,
        linebreak: u8,
    ) -> std::io::Result<Stats> {
        let mut state = Self::State::initial_state();
        let mut buff = [0; BUFFER_SIZE];
        loop {
            let read = reader.read(&mut buff)?;
            if read == 0 {
                return Ok(state.result());
            }
            state = self.run(state, &buff[0..read], linebreak);
        }
    }
}
