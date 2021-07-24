
use crate::stats::Stats;
use crate::stats::automata::response::Response;
use crate::stats::automata::posix::State::{Carriage, NewLine, Nil, Word};

/// Represents a node on the automata. The current automata desing can be
/// studied on .github/desing/Automata.drawio. The automata allows partial
/// computation by providing a `PartialResponse` instance
enum State {
    Nil,      // Espacios y nodo inicial
    NewLine,  // se encuentra un \n
    Carriage, // Se encuentra un \r
    Word,     // Se encuentra algún carácter válido
              // utf
}
impl Default for State {
    /// Initial node
    fn default() -> Self {
        State::Nil
    }
}

/// Represents progress for a finite automata. Can be converted into a final
/// result by using the `result()` function
#[derive(Default)]
pub struct PartialResponse(State, Stats);

impl Response for PartialResponse{
    /// Initial state for the automata
    fn initial_state() -> PartialResponse {
        PartialResponse::default()
    }
    /// Transforms a `PartialResponse` into `Stats`
    fn result(self) -> Stats {
        // TODO add utf support
        let PartialResponse(state, mut stats) = self;
        match state {
            Carriage => {
                stats.lines += 1;
                stats
            }
            Word => {
                stats.words += 1;
                stats
            }
            _ => stats,
        }
    }
}
/// Represents a Finite Deterministic Automata which fetchs it's input from a
/// given tape
pub struct Posix;
impl Posix {
    /// Runs the automata over the given tape, generating a partial response
    pub fn run(partial: PartialResponse, tape: &[u8]) -> PartialResponse {
        // TODO doest work as expected
        // Bytes: works
        // Characters: No
        // Words: No
        // Lines: No
        fn newline(mut s: Stats) -> Stats {
            s.lines += 1;
            s
        }
        fn newword(mut s: Stats) -> Stats {
            s.words += 1;
            s
        }
        fn newchar(mut s :Stats)-> Stats {
            s.characters+=1;
            s
        }

        let result: (State, Stats) = tape.iter().fold((partial.0, partial.1), |s, c| {
            let (state, mut stats) = s;
            // One byte read
            stats.bytes+=1;
            match state {
                State::Nil => match c {
                    b'\r' => (Carriage, stats),
                    b'\n' => (NewLine, newline(stats)),
                    b' ' => (Nil, stats),
                    _ => (Word, newchar(stats)),
                },
                State::Carriage => {
                    let temp = newline(stats);
                    match c {
                        b'\r' => (Carriage, temp),
                        b'\n' => (NewLine, temp),
                        b' ' => (Nil, temp),
                        _ => (Word, newchar(temp)),
                    }
                }
                State::NewLine => {
                    let temp = newline(stats);
                    match c {
                        b'\r' => (Carriage, temp),
                        b'\n' => (NewLine, temp),
                        b' ' => (Nil, temp),
                        _ => (Word, newchar(temp)),
                    }
                }
                State::Word => match c {
                    b'\r' => (Carriage, newword(stats)),
                    b'\n' => (NewLine, newword(newline(stats))),
                    b' ' => (Nil, stats),
                    _ => (Word, newchar(stats)),
                },
            }
        });

        PartialResponse {
            0: result.0,
            1: result.1,
        }
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
}