use crate::cw::stats::Stats;

/// An automaton partial state
pub trait PartialState
where
    Self: Default,
{
    fn initial_state() -> Self {
        Self::default()
    }
    /// Transforms a `PartialState` into `Stats`
    fn result(self) -> Stats;
}
