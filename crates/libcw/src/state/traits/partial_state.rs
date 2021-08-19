/// Represents a state that is not definitive but can be transformed into some
/// kind of output state. For example, it may be used to transform a
/// partial byte counter state into a final (usize) value
pub trait PartialState {
    type Output;
    fn output(&self) -> Self::Output;
}
