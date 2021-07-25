use crate::stats::Stats;
use crate::stats::automata::partial_response::PartialResponse;
use unicode_general_category::get_general_category;
use unicode_general_category::GeneralCategory::*;

/// UTF char uses 4 bytes at most
type UTFCharBuff = [u8;4];

// If we are on a word or not
type OnWord = bool;

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
    pub fn decode(byte:&u8) -> State {
        let four = 0b11110000;      // 11110uuu 10uuzzzz 10yyyyyy 10xxxxxx
        let three = 0b11100000;     // 1110zzzz 10yyyyyy 10xxxxxx
        let two = 0b11000000;       // 110yyyyy 10xxxxxx

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
pub struct PosixPartialState(State, OnWord, Stats, UTFCharBuff);

impl PartialResponse for PosixPartialState {
    /// Initial state for the automata
    fn initial_state() -> PosixPartialState {
        PosixPartialState::default()
    }
    /// Transforms a `PosixPartialState` into `Stats`
    fn result(self) -> Stats {
        let PosixPartialState(state,onword,mut stats,buff) = self;
        if onword {
            stats.words+=1;
        }
        stats
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
        // Lines: Works
        let PosixPartialState(mut expect, mut onword, mut stats, mut buff) =
            partial;
        match expect {
            State::New => { // -> One,Two,Three,Four
                // Done
                expect = State::decode(char);
                let state = PosixPartialState(expect,onword,stats,buff);
                PosixUTF8::compute(state,char)
            }
            State::One => { // -> New
                stats.bytes+=1;
                buff[0] = *char;
                expect = State::New;

                // If end we need to add one char to the count (it represents
                // before we had a char). The program does not count the last
                // char. Instead, it counts from zero
                // - Reset buffer to empty
                // - Write on buff [0]
                // update stats
                let opt_character= char::from_u32(u32::from_le_bytes(buff));
                if let Some(char) = opt_character {
                    stats.characters+=1;
                    match char {
                        '\n' => {
                            stats.lines+=1;
                            if onword {
                                stats.words+=1;
                            }
                            onword = false;
                        }
                        x=> {
                            match get_general_category(x) {
                                LowercaseLetter | UppercaseLetter | ModifierLetter |
                                TitlecaseLetter | OtherLetter => {
                                    onword = true;
                                },
                                otherwise=>{
                                    onword = false;
                                    stats.words+=1;
                                }
                            }
                        }
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

                PosixPartialState(expect,onword,stats,buff)
            }
            State::Two => {
                stats.bytes+=1;
                buff[1] = *char;
                expect = State::One;
                PosixPartialState(expect,onword,stats,buff)

            }
            State::Three => {
                stats.bytes+=1;
                buff[2] = *char;
                expect = State::Two;
                PosixPartialState(expect,onword,stats,buff)
            }
            State::Four => {
                stats.bytes+=1;
                buff[3] = *char;
                expect = State::Three;
                PosixPartialState(expect,onword,stats,buff)
            }
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

    // Macro?
    macro_rules! isspace {
        ($char:expr) => {
            ($char == 0x9 ) || ($char == 0x20) || ($char >= 0xA && $char <=0xD)
        }
    }
}