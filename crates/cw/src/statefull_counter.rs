use dyn_clone::DynClone;
use libcw::{
    counter::{Collapse, Counter},
    Stats,
};

pub trait Eat: DynClone {
    fn eat(&mut self, tape: &[u8]);
    fn terminate(&mut self, collapsable: Stats) -> Stats;
}

#[derive(Debug, Clone)]
pub struct StatsCounter<C, S> {
    counter: C,
    state: S,
}
impl<C, S> StatsCounter<C, S> {
    pub fn new(counter: C, state: S) -> Self {
        Self { counter, state }
    }
}

impl<C, S, O> Eat for StatsCounter<C, S>
where
    C: Counter<State = S, Output = O> + Clone,
    S: Clone,
    O: Collapse<Stats>,
{
    fn eat(&mut self, tape: &[u8]) {
        self.state = self.counter.parse(tape, self.state.clone())
    }

    fn terminate(&mut self, collapsable: Stats) -> Stats {
        self.counter
            .terminate(self.state.clone())
            .collapse(collapsable)
    }
}
