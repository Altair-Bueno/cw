
use crate::stats::automata::automata::Automata;
use crate::stats::automata::partial_state::PartialState;
use crate::stats::automata::OnWord;
use crate::stats::Stats;
use crate::isspace;

/// UTF char uses 4 bytes at most
type UTFCharBuff = [u8; 4];

enum State {
    New,
    One,
    Two,
    Three,
    Four,
}
impl Default for State {
    fn default() -> Self {
        State::New
    }
}

impl State {
    pub fn decode(byte: &u8) -> State {
        let four = 0b11110000; // 11110uuu 10uuzzzz 10yyyyyy 10xxxxxx
        let three = 0b11100000; // 1110zzzz 10yyyyyy 10xxxxxx
        let two = 0b11000000; // 110yyyyy 10xxxxxx

        if byte & four == four {
            State::Four
        } else if byte & three == three {
            State::Three
        } else if byte & two == two {
            State::Two
        } else {
            State::One
        }
    }
}

/// Represents progress for a finite automata. Can be converted into a final
/// result by using the `result()` function
#[derive(Default)]
pub struct PosixUTF8PartialState(State, OnWord, Stats, UTFCharBuff);

impl PartialState for PosixUTF8PartialState {
    /// Initial state for the automata
    fn initial_state() -> PosixUTF8PartialState {
        PosixUTF8PartialState::default()
    }
    /// Transforms a `PosixUTF8PartialState` into `Stats`
    fn result(self) -> Stats {
        let PosixUTF8PartialState(state, onword, mut stats, buff) = self;
        if onword {
            stats.words += 1;
        }
        stats
    }
}

/// Represents a Finite Deterministic Automata which fetchs it's input from a
/// given tape
pub struct PosixUTF8;

impl Automata for PosixUTF8 {
    type State = PosixUTF8PartialState;

    fn run(&self, partial: Self::State, tape: &[u8]) -> Self::State {
        tape.iter().fold(partial, PosixUTF8::compute)
    }
}

impl PosixUTF8 {
    /// Runs the automata over the given tape, generating a partial response
    fn compute(partial: PosixUTF8PartialState, char: &u8) -> PosixUTF8PartialState {
        // TODO doest work as expected
        // Bytes: works
        // Characters: No
        // Words: No
        // Lines: Works
        let PosixUTF8PartialState(mut expect, mut onword, mut stats, mut buff) = partial;
        match expect {
            State::New => {
                // -> One,Two,Three,Four
                // Done
                expect = State::decode(char);
                let state = PosixUTF8PartialState(expect, onword, stats, buff);
                PosixUTF8::compute(state, char)
            }
            State::One => {
                // -> New
                stats.bytes += 1;
                buff[0] = *char;

                // If end we need to add one char to the count (it represents
                // before we had a char). The program does not count the last
                // char. Instead, it counts from zero
                // - Reset buffer to empty
                // - Write on buff [0]
                // update stats
                let opt_character = char::from_u32(u32::from_le_bytes(buff));
                if let Some(char) = opt_character {
                    stats.characters += 1;
                    match char {
                        '\n' => {
                            stats.lines += 1;
                            if onword {
                                stats.words += 1;
                            }
                            onword = false;
                        }
                        x if isspace!(x as u32) => {
                            if onword {
                                stats.words += 1;
                                onword = false;
                            }
                        }
                        _ => onword = true,
                    }
                    // Character read
                    // update onword
                    // update stats
                } else {
                    // Something went wrong
                    // update onword
                    // update stats
                    onword = false;
                }

                buff.fill(0);
                expect = State::New;

                PosixUTF8PartialState(expect, onword, stats, buff)
            }
            State::Two => {
                stats.bytes += 1;
                buff[1] = *char;
                expect = State::One;
                PosixUTF8PartialState(expect, onword, stats, buff)
            }
            State::Three => {
                stats.bytes += 1;
                buff[2] = *char;
                expect = State::Two;
                PosixUTF8PartialState(expect, onword, stats, buff)
            }
            State::Four => {
                stats.bytes += 1;
                buff[3] = *char;
                expect = State::Three;
                PosixUTF8PartialState(expect, onword, stats, buff)
            }
        }
    }
}
