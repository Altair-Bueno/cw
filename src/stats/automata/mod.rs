pub mod ascii;
pub mod automata_config;
pub mod trait_automata;
mod encoding;
mod line_break;
pub(crate) mod trait_partial_state;
pub mod utf8;

// If we are on a word or not
type OnWord = bool;
// Macro?

#[macro_export]
macro_rules! isspace {
    ($char:expr) => {
        ($char == 0x9) || ($char == 0x20) || ($char >= 0xA && $char <= 0xD)
    };
}
