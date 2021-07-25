pub mod posix_utf8;
pub(crate) mod partial_response;
mod posix_ascii;
mod automata;

// If we are on a word or not
type OnWord = bool;