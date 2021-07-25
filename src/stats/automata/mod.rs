pub mod ascii;
pub mod automata;
pub mod automata_trait;
mod encoding;
mod line_break;
pub(crate) mod partial_state;
pub mod utf8;

// If we are on a word or not
type OnWord = bool;
