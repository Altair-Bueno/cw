use crate::cw_lib::func::lines_state::LinesState;
use crate::cw_lib::func::words_state::WordsState;
use crate::cw_lib::func::bytes_state::BytesState;
use crate::cw_lib::func::max_length::MaxLengthState;
use crate::cw_lib::func::chars_state::CharState;

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