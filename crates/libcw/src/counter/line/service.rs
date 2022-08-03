use tower_layer::Layer;

use crate::counter::Collapse;
use crate::counter::Counter;

use super::*;

#[derive(Debug, Default, Clone)]
pub struct LineCounterServiceState<S> {
    inner: S,
    state: LineCounterState,
}

#[derive(Debug, Default, Clone)]
pub struct LineCounterServiceOutput<S> {
    inner: S,
    output: usize,
}

#[derive(Debug, Default, Clone)]
pub struct Lines(usize);

#[cfg(feature = "anymap")]
impl<S> Collapse<anymap::AnyMap> for LineCounterServiceOutput<S>
where
    S: Collapse<anymap::AnyMap>,
{
    fn collapse(self, mut colapsable: anymap::AnyMap) -> anymap::AnyMap {
        colapsable.insert(Lines(self.output));
        self.inner.collapse(colapsable)
    }
}

#[derive(Debug, Default, Clone)]
pub struct LineCounterService<S> {
    inner: S,
    counter: LineCounter,
}

impl<'t, S> Counter<&'t [u8]> for LineCounterService<S>
where
    S: Counter<&'t [u8]>,
{
    type State = LineCounterServiceState<S::State>;

    type Output = LineCounterServiceOutput<S::Output>;

    fn parse(&self, input: &'t [u8], state: Self::State) -> Self::State {
        LineCounterServiceState {
            inner: self.inner.parse(input, state.inner),
            state: self.counter.parse(input, state.state),
        }
    }

    fn terminate(&self, state: Self::State) -> Self::Output {
        LineCounterServiceOutput {
            inner: self.inner.terminate(state.inner),
            output: self.counter.terminate(state.state),
        }
    }
}

impl<S> Layer<S> for LineCounter {
    type Service = LineCounterService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        LineCounterService {
            inner,
            counter: self.clone(),
        }
    }
}
