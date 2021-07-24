use crate::stats::Stats;
use crate::stats::automata::partial_response::PartialResponse;

/// UTF char uses 4 bytes at most
type UTFCharBuff = [u8;4];

// If we are on a word or not
type OnWord = bool;

enum UTFSize {
    One,
    Two,
    Three,
    Four,
}
impl Default for UTFSize {
    fn default() -> Self {
        UTFSize::One
    }
}

impl UTFSize {
    pub fn utf_size(byte:u8) -> UTFSize{
        let four = 0b11110000;
        let three = 0b11100000;
        let two = 0b11000000;

        if byte &  four == four {
            UTFSize::Four
        } else if byte & three == three {
            UTFSize::Three
        } else if byte & two == two {
            UTFSize::Two
        } else {
            UTFSize::One
        }
    }
}

/// Represents progress for a finite automata. Can be converted into a final
/// result by using the `result()` function
#[derive(Default)]
pub struct PosixPartialState(UTFSize, OnWord, Stats, UTFCharBuff);

impl PartialResponse for PosixPartialState {
    /// Initial state for the automata
    fn initial_state() -> PosixPartialState {
        PosixPartialState::default()
    }
    /// Transforms a `PosixPartialState` into `Stats`
    fn result(self) -> Stats {
        todo!()
    }
}

/// Represents a Finite Deterministic Automata which fetchs it's input from a
/// given tape
pub struct PosixUTF8;
impl PosixUTF8 {
    /// Runs the automata over the given tape, generating a partial response
    pub fn run(partial: PosixPartialState, tape: &[u8]) -> PosixPartialState {
        tape.iter().fold(partial, PosixUTF8::compute)
    }

    fn compute(partial:PosixPartialState,char:&u8) -> PosixPartialState {
        // TODO doest work as expected
        // Bytes: works
        // Characters: No
        // Words: No
        // Lines: No

        let PosixPartialState(expect, onword, mut stats, mut buff) = partial;
        // New byte. Update
        stats.bytes+=1;
        let onword_next = todo!();
        let expect_next = todo!();
        PosixPartialState(expect_next,onword_next,stats,buff)
    }
}

mod utils {
    /// Defined on C95: wctype.h
    /// https://en.cppreference.com/w/c/string/wide/iswspace
    pub fn isspace(char:u8) -> bool {
        (char == 0x9 ) || (char == 0x20) || (char >= 0xA && char <= 0xD)
    }
    /// Defined on C95: wctype.h
    /// https://en.cppreference.com/w/c/string/wide/iswspace
    pub fn isalpha (char :u8) ->bool {
        (char >= 0x41 && char <=0x5A) || (char>=0x61 && char <= 0x7A)
    }

    // Macro?
    macro_rules! isspace {
        ($char:expr) => {
            ($char == 0x9 ) || ($char == 0x20) || ($char >= 0xA && $char <=0xD)
        }
    }
}