use crate::cw_lib::state::traits::{PartialState, Compute};

// TODO
#[derive(Default,Copy, Clone)]
pub struct CharState{

}
impl PartialState for CharState {
    type Output = u32;

    fn output(&self)->Self::Output {
        todo!()
    }
}
impl Compute for CharState {
    fn compute(self, tape: &[u8]) -> Self {
        todo!()
    }
}