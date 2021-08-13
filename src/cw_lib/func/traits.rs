pub trait PartialState {
    fn output(&self)->Result<u32,String>;
}

pub trait Compute {
    fn compute(self,tape: &[u8]) -> Self;
}