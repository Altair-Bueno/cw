use crate::stats::Stats;

pub trait PartialState {
    fn initial_state() -> Self;
    /// Transforms a `PartialState` into `Stats`
    fn result(self) -> Stats ;
}