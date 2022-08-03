use tower_layer::Layer;

use crate::counter::Collapse;
use crate::counter::Counter;

use super::{ByteCounter, ByteCounterState};
use anymap::AnyMap;

#[derive(Debug, Default, Clone)]
pub struct ByteCounterServiceState<S> {
    inner: S,
    state: ByteCounterState,
}

#[derive(Debug, Default, Clone)]
pub struct ByteCounterServiceOutput<S> {
    inner: S,
    output: usize,
}

pub struct Bytes(usize);

impl<S> Collapse<AnyMap> for ByteCounterServiceOutput<S>
where
    S: Collapse<AnyMap>,
{
    fn collapse(self, mut colapsable: AnyMap) -> AnyMap {
        colapsable.insert(Bytes(self.output));
        self.inner.collapse(colapsable)
    }
}

#[derive(Debug, Default, Clone)]
pub struct ByteCounterService<S> {
    inner: S,
    counter: ByteCounter,
}

impl<'t, S> Counter<&'t [u8]> for ByteCounterService<S>
where
    S: Counter<&'t [u8]>,
{
    type State = ByteCounterServiceState<S::State>;

    type Output = ByteCounterServiceOutput<S::Output>;

    fn parse(&self, input: &'t [u8], state: Self::State) -> Self::State {
        ByteCounterServiceState {
            inner: self.inner.parse(input, state.inner),
            state: self.counter.parse(input, state.state),
        }
    }

    fn terminate(&self, state: Self::State) -> Self::Output {
        ByteCounterServiceOutput {
            inner: self.inner.terminate(state.inner),
            output: self.counter.terminate(state.state),
        }
    }
}

impl<S> Layer<S> for ByteCounter {
    type Service = ByteCounterService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        ByteCounterService {
            inner,
            counter: self.clone(),
        }
    }
}
