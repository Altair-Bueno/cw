use crate::stats::Stats;

pub trait PartialResponse {
    fn initial_state() -> Self;
    /// Transforms a `PartialResponse` into `Stats`
    fn result(self) -> Stats ;
}