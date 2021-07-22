use crate::stats::automata::State::{Carriage, NewLine, Nil, Word};
use crate::stats::Stats;

/// Represents a node on the automata. The current automata desing can be
/// studied on .github/desing/Automata.drawio. The automata allows partial
/// computation by providing a `PartialResponse` instance
enum State {
    Nil,       // Espacios y nodo inicial
    NewLine,   // se encuentra un \n
    Carriage,  // Se encuentra un \r
    Word,      // Se encuentra algún carácter válido
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

impl PartialResponse {
    /// Initial state for the automata
    pub fn initial_state () -> PartialResponse {
        PartialResponse::default()
    }
    /// Transforms a `PartialResponse` into `Stats`
    pub fn result(self) -> Stats {
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
pub struct Automata;
impl Automata {
    /// Runs the automata over the given tape, generating a partial response
    pub fn run(partial: PartialResponse, tape: &[u8]) -> PartialResponse {
        fn newline(mut s: Stats) -> Stats {
            s.lines += 1;
            s.bytes += 1;
            s
        }
        fn newword(mut s: Stats) -> Stats {
            s.bytes += 1;
            s.words += 1;
            s
        }
        fn addbyte(mut s: Stats) -> Stats {
            s.bytes += 1;
            s
        }

        let result: (State, Stats) = tape.iter().fold((partial.0, partial.1), |s, c| {
            let (state, stats) = s;
            match state {
                State::Nil => match c {
                    b'\r' => (Carriage, stats),
                    b'\n' => (NewLine, newline(stats)),
                    b' ' => (Nil, stats),
                    _ => (Word, stats),
                },
                State::Carriage => {
                    let temp = newline(stats);
                    match c {
                        b'\r' => (Carriage, temp),
                        b'\n' => (NewLine, temp),
                        b' ' => (Nil, temp),
                        _ => (Word, temp),
                    }
                }
                State::NewLine => {
                    let temp = newline(stats);
                    match c {
                        b'\r' => (Carriage, temp),
                        b'\n' => (NewLine, temp),
                        b' ' => (Nil, temp),
                        _ => (Word, temp),
                    }
                }
                State::Word => match c {
                    b'\r' => (Carriage, newword(stats)),
                    b'\n' => (NewLine, newword(newline(stats))),
                    b' ' => (Nil, stats),
                    _ => (Word, addbyte(stats)),
                },
            }
        });

        PartialResponse {
            0: result.0,
            1: result.1,
        }
    }

}
