use crate::cw_lib::state::lines_state::LinesState;
use crate::cw_lib::state::words_state::WordsState;
use crate::cw_lib::state::bytes_state::BytesState;
use crate::cw_lib::state::max_length::MaxLengthState;
use crate::cw_lib::state::chars_state::CharState;
use crate::cw_lib::state::traits::PartialState;
use crate::cw_lib::stats::Stats;

pub mod traits;
pub mod lines_state;
pub mod words_state;
pub mod bytes_state;
pub mod max_length;
mod chars_state;

pub type State = (
    LinesState,
    WordsState,
    CharState,
    BytesState,
    MaxLengthState,
);

impl PartialState for State {
    type Output = Stats;
    fn output(&self)->Result<Self::Output,String>{
        todo!()
    }
}