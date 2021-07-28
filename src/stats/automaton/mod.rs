// Private members
pub mod automaton_utf8;
pub mod trait_automaton;
pub mod trait_partial_state;

// Private types and def
// If we are on a word or not
type OnWord = bool;
/*// Macro version
#[macro_export]
macro_rules! isspace {
    ($char:expr) => {
        ($char == 0x9) || ($char == 0x20) || ($char >= 0xA && $char <= 0xD)
    };
}*/
#[inline(always)]
fn isspace(char :u32) -> bool {
    (char == 0x9) || (char == 0x20) || (char >= 0xA && char <= 0xD)
}