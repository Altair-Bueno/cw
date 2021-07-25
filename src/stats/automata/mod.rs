pub use automata::Automata as Automata;

mod automata;
pub(crate) mod partial_state;
pub mod posix_ascii;
pub mod posix_utf8;

// If we are on a word or not
type OnWord = bool;
