use crate::stats::Stats;

pub trait Response {
    fn initial_state() -> Self;
    /// Transforms a `PartialResponse` into `Stats`
    fn result(self) -> Stats ;
}