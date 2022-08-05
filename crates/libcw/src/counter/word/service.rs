use std::ops::Deref;

use tower_layer::Layer;

use crate::counter::service::*;
use crate::counter::Collapse;
use crate::counter::Counter;

use super::*;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Default, Clone)]
pub struct Words(pub usize);

impl Deref for Words {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(feature = "stats")]
impl<I> Collapse<crate::Stats> for CounterServiceOutput<Words, I>
where
    I: Collapse<crate::Stats>,
{
    fn collapse(self, mut colapsable: crate::Stats) -> crate::Stats {
        colapsable.words = Some(*self.output);
        self.inner.collapse(colapsable)
    }
}

impl<S> Counter for CounterService<WordCounter, S>
where
    S: Counter,
{
    type State = CounterServiceState<WordCounterState, S::State>;

    type Output = CounterServiceOutput<Words, S::Output>;

    fn parse(&self, input: &[u8], state: Self::State) -> Self::State {
        Self::State {
            inner: self.inner.parse(input, state.inner),
            state: self.counter.parse(input, state.state),
        }
    }

    fn terminate(&self, state: Self::State) -> Self::Output {
        Self::Output {
            inner: self.inner.terminate(state.inner),
            output: Words(self.counter.terminate(state.state)),
        }
    }
}

impl<S> Layer<S> for WordCounter {
    type Service = CounterService<WordCounter, S>;

    fn layer(&self, inner: S) -> Self::Service {
        Self::Service {
            counter: self.clone(),
            inner,
        }
    }
}
