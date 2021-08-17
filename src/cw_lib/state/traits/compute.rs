
/// Represents an state that could carry on computations
pub trait Compute {
    fn compute(self, tape: &[u8]) -> Self;
}
