use crate::cw_lib::state::traits::PartialState;

// TODO
#[derive(Default,Copy, Clone)]
pub struct CharState{

}
impl PartialState for CharState {
    type Output = u32;

    fn output(&self) -> Result<Self::Output, String> {
        todo!()
    }
}