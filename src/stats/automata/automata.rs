use crate::stats::automata::partial_state::PartialState;
use crate::stats::Stats;
use std::io::BufRead;

const BUFFER_SIZE: usize = 16 * 1024; // 16KB
pub trait Automata {
    type State: PartialState + Sized;

    fn run(&self, partial: Self::State, tape: &[u8]) -> Self::State;

    fn stats_from_bufread(&self, mut reader: Box<dyn BufRead>) -> std::io::Result<Stats> {
        let mut state = Self::State::initial_state();
        // TODO use a single buffer for all operations instead
        let mut buff = [0; BUFFER_SIZE];
        loop {
            let read = reader.read(&mut buff)?;
            if read == 0 {
                return Ok(state.result());
            }
            state = self.run(state, &buff[0..read]);
        }
    }
}
