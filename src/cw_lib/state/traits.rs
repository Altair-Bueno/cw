/// Represents a state that is not definitive
pub trait PartialState {
    type Output;
    fn output(&self)->Self::Output;
}
/// Represents an state that could carry on computations
pub trait Compute {
    fn compute(self,tape: &[u8]) -> Self;
}