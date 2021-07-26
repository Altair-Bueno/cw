// Private members
pub mod automata_utf8;
pub mod posix_ascii;
pub mod trait_automata;
pub mod trait_partial_state;

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
