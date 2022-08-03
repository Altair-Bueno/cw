use tower_layer::Layer;

use crate::counter::Collapse;
use crate::counter::Counter;

use super::*;

#[derive(Debug, Default, Clone)]
pub struct WordCounterServiceState<S> {
    inner: S,
    state: WordCounterState,
}

#[derive(Debug, Default, Clone)]
pub struct WordCounterServiceOutput<S> {
    inner: S,
    output: usize,
}

#[derive(Debug, Default, Clone)]
pub struct Words(usize);

#[cfg(feature = "anymap")]
impl<S> Collapse<anymap::AnyMap> for WordCounterServiceOutput<S>
where
    S: Collapse<anymap::AnyMap>,
{
    fn collapse(self, mut colapsable: anymap::AnyMap) -> anymap::AnyMap {
        colapsable.insert(Words(self.output));
        self.inner.collapse(colapsable)
    }
}

#[derive(Debug, Default, Clone)]
pub struct WordCounterService<S> {
    inner: S,
    counter: WordCounter,
}

impl<'t, S> Counter<&'t [u8]> for WordCounterService<S>
where
    S: Counter<&'t [u8]>,
{
    type State = WordCounterServiceState<S::State>;

    type Output = WordCounterServiceOutput<S::Output>;

    fn parse(&self, input: &'t [u8], state: Self::State) -> Self::State {
        WordCounterServiceState {
            inner: self.inner.parse(input, state.inner),
            state: self.counter.parse(input, state.state),
        }
    }

    fn terminate(&self, state: Self::State) -> Self::Output {
        WordCounterServiceOutput {
            inner: self.inner.terminate(state.inner),
            output: self.counter.terminate(state.state),
        }
    }
}

impl<S> Layer<S> for WordCounter {
    type Service = WordCounterService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        WordCounterService {
            inner,
            counter: self.clone(),
        }
    }
}
