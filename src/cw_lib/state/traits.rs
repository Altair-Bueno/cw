pub trait PartialState {
    type Output;
    fn output(&self)->Result<Self::Output,String>;
}

pub trait Compute {
    fn compute(self,tape: &[u8]) -> Self;
}