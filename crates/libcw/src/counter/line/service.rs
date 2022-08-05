use std::ops::Deref;

use tower_layer::Layer;

use crate::counter::service::*;
use crate::counter::Collapse;
use crate::counter::Counter;

use super::*;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Default, Clone)]
pub struct Lines(pub usize);

impl Deref for Lines {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(feature = "stats")]
impl<I> Collapse<crate::Stats> for CounterServiceOutput<Lines, I>
where
    I: Collapse<crate::Stats>,
{
    fn collapse(self, mut colapsable: crate::Stats) -> crate::Stats {
        colapsable.lines = Some(*self.output);
        self.inner.collapse(colapsable)
    }
}

impl<S> Counter for CounterService<LineCounter, S>
where
    S: Counter,
{
    type State = CounterServiceState<LineCounterState, S::State>;

    type Output = CounterServiceOutput<Lines, S::Output>;

    fn parse(&self, input: &[u8], state: Self::State) -> Self::State {
        Self::State {
            inner: self.inner.parse(input, state.inner),
            state: self.counter.parse(input, state.state),
        }
    }

    fn terminate(&self, state: Self::State) -> Self::Output {
        Self::Output {
            inner: self.inner.terminate(state.inner),
            output: Lines(self.counter.terminate(state.state)),
        }
    }
}

impl<S> Layer<S> for LineCounter {
    type Service = CounterService<LineCounter, S>;

    fn layer(&self, inner: S) -> Self::Service {
        Self::Service {
            counter: self.clone(),
            inner,
        }
    }
}
