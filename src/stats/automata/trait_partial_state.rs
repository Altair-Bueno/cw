use crate::stats::stats::Stats;

pub trait PartialState where Self : Default{
    fn initial_state() -> Self {
        Self::default()
    }
    /// Transforms a `PartialState` into `Stats`
    fn result(self) -> Stats;
}
