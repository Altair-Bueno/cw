use std::ops::Deref;

use tower_layer::Layer;

use crate::counter::service::*;
use crate::counter::Collapse;
use crate::counter::Counter;

use super::*;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Default, Clone)]
pub struct Bytes(pub usize);

impl Deref for Bytes {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(feature = "stats")]
impl<I> Collapse<crate::Stats> for CounterServiceOutput<Bytes, I>
where
    I: Collapse<crate::Stats>,
{
    fn collapse(self, mut colapsable: crate::Stats) -> crate::Stats {
        colapsable.bytes = Some(*self.output);
        self.inner.collapse(colapsable)
    }
}

impl<S> Counter for CounterService<ByteCounter, S>
where
    S: Counter,
{
    type State = CounterServiceState<ByteCounterState, S::State>;

    type Output = CounterServiceOutput<Bytes, S::Output>;

    fn parse(&self, input: &[u8], state: Self::State) -> Self::State {
        Self::State {
            inner: self.inner.parse(input, state.inner),
            state: self.counter.parse(input, state.state),
        }
    }

    fn terminate(&self, state: Self::State) -> Self::Output {
        Self::Output {
            inner: self.inner.terminate(state.inner),
            output: Bytes(self.counter.terminate(state.state)),
        }
    }
}

impl<S> Layer<S> for ByteCounter {
    type Service = CounterService<ByteCounter, S>;

    fn layer(&self, inner: S) -> Self::Service {
        Self::Service {
            counter: self.clone(),
            inner,
        }
    }
}
