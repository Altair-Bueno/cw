/// Represents an state that could carry on computations. Is it used to
/// produce intermediate states between tapes. For example, it can be used
/// for a bytecounter to count bytes on a tape a, then continue computation over
/// a second tape b

pub trait Compute {
    fn compute(self, tape: &[u8]) -> Self;
}
