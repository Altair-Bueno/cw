/// Represents a state that is not definitive
pub trait PartialState {
    type Output;
    fn output(&self) -> Self::Output;
}
