use anymap::AnyMap;
use libcw::counter::{Collapse, Counter};

pub trait Eater<TAPE, COLLAPSABLE> {
    fn eat(&mut self, tape: TAPE);
    fn result(&self, collapsable: COLLAPSABLE) -> COLLAPSABLE;
}

#[derive(Clone, Debug)]
pub struct AbstractEater<S, C> {
    state: S,
    counter: C,
}

impl<S, C> AbstractEater<S, C> {
    pub fn new(state: S, counter: C) -> Self {
        Self { state, counter }
    }
}

impl<'t, S, C, O> Eater<&'t [u8], AnyMap> for AbstractEater<S, C>
where
    C: Counter<&'t [u8], State = S, Output = O>,
    S: Clone,
    O: Collapse<AnyMap>,
{
    fn eat(&mut self, tape: &'t [u8]) {
        self.state = self.counter.parse(tape, self.state.clone())
    }

    fn result(&self, collapsable: AnyMap) -> AnyMap {
        self.counter
            .terminate(self.state.clone())
            .collapse(collapsable)
    }
}
