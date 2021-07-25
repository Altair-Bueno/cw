mod automata;
pub(crate) mod partial_state;
mod posix_ascii;
pub mod posix_utf8;
pub use automata::Automata as Automata;

// If we are on a word or not
type OnWord = bool;
