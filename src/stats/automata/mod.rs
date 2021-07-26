// Private members
mod automata_utf8;
mod encoding;
mod line_break;
mod posix_ascii;
mod trait_automata;
mod trait_partial_state;

// Public API
pub mod automata_config;

// Private types and def
// If we are on a word or not
type OnWord = bool;
// Macro?
#[macro_export]
macro_rules! isspace {
    ($char:expr) => {
        ($char == 0x9) || ($char == 0x20) || ($char >= 0xA && $char <= 0xD)
    };
}
