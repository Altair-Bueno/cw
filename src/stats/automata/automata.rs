use crate::stats::automata::partial_state::PartialState;

pub trait Automata {
    type State: PartialState + Sized;
    fn run(&self,partial: Self::State, tape: &[u8]) -> Self::State;
    fn initial_state(&self) ->Self::State {
        Self::State::initial_state()
    }
}
