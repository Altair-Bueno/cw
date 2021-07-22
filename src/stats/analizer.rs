use crate::stats::analizer::State::{Carriage, NewLine, Nil, Word};
use crate::stats::Stats;

// Newline: se encuentra un \n
// Carriage: Se encuentra un \r
// Word: Se encuentra algún carácter válido
//
enum State {
    Nil,
    NewLine,
    Carriage,
    Word,
    // utf
}
impl Default for State {
    fn default() -> Self {
        State::Nil
    }
}

pub struct PartialResponse(State, Stats);

impl PartialResponse {
    fn result(self) -> Stats {
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

pub fn automata(partial: PartialResponse, tape: &[u8]) -> PartialResponse {
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
