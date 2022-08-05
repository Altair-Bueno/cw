#[derive(Debug, Default, Clone)]
pub struct CounterServiceState<S, I> {
    pub(super) state: S,
    pub(super) inner: I,
}

#[derive(Debug, Default, Clone)]
pub struct CounterService<C, I> {
    pub(super) counter: C,
    pub(super) inner: I,
}

#[derive(Debug, Default, Clone)]
pub struct CounterServiceOutput<O, I> {
    pub(super) output: O,
    pub(super) inner: I,
}
