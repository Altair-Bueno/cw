use crate::stats::automata::partial_state::PartialState;
use crate::stats::automata::{Automata, OnWord};
use crate::stats::Stats;

#[macro_use]
mod utils {
    /// Defined on C95: wctype.h
    /// https://en.cppreference.com/w/c/string/wide/iswspace
    #[cfg(unused)]
    pub fn isspace(char: u8) -> bool {
        (char == 0x9) || (char == 0x20) || (char >= 0xA && char <= 0xD)
    }
    /// Defined on C95: wctype.h
    /// https://en.cppreference.com/w/c/string/wide/iswspace
    #[cfg(unused)]
    pub fn isalpha(char: u8) -> bool {
        (char >= 0x41 && char <= 0x5A) || (char >= 0x61 && char <= 0x7A)
    }

    // Macro?
    macro_rules! isspace {
        ($char:expr) => {
            ($char == 0x9) || ($char == 0x20) || ($char >= 0xA && $char <= 0xD)
        };
    }
}

#[derive(Default)]
pub struct PosixASCIIPartialState(OnWord, Stats);

impl PartialState for PosixASCIIPartialState {
    fn initial_state() -> Self {
        PosixASCIIPartialState::default()
    }

    fn result(self) -> Stats {
        let PosixASCIIPartialState(onword, mut stats) = self;

        if onword {
            stats.words += 1;
        }
        stats
    }
}

pub struct PosixASCII;

impl Automata for PosixASCII {
    type State = PosixASCIIPartialState;

    fn run(&self, partial: Self::State, tape: &[u8]) -> Self::State {
        tape.iter().fold(partial, PosixASCII::compute)
    }
}

impl PosixASCII {
    fn compute(partial: PosixASCIIPartialState, char: &u8) -> PosixASCIIPartialState {
        let PosixASCIIPartialState(mut onword, mut stats) = partial;
        stats.characters += 1;
        stats.bytes += 1;
        match char {
            b'\n' => {
                if onword {
                    stats.words += 1;
                    onword = false;
                }
                stats.lines += 1;
            }
            x if isspace!(*x) => {
                if onword {
                    stats.words += 1;
                    onword = false;
                }
            }
            _ => onword = true,
        }
        PosixASCIIPartialState(onword, stats)
    }
}
