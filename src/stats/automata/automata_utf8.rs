use crate::isspace;
use crate::stats::automata::trait_automata::Automata;
use crate::stats::automata::trait_partial_state::PartialState;
use crate::stats::automata::OnWord;
use crate::stats::stats::Stats;
use std::cmp::max;

/// UTF char uses 4 bytes at most
type UTFCharBuff = [u8; 4];
type CurrentLength = u32;

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
pub struct UTF8PartialState(State, OnWord ,CurrentLength , Stats, UTFCharBuff);

impl PartialState for UTF8PartialState {
    /// Transforms a `UTF8PartialState` into `Stats`
    fn result(self) -> Stats {
        let UTF8PartialState(_, onword, legth, mut stats, _) = self;
        if onword {
            stats.words += 1;
        }
        stats.legth = max(stats.legth,legth);
        stats
    }
}

/// Represents a Finite Deterministic Automata which fetchs it's input from a
/// given tape
pub struct AutomataUTF8;

impl Automata for AutomataUTF8 {
    type State = UTF8PartialState;

    fn run(&self, partial: Self::State, tape: &[u8],linebreak:char) ->
                                                                 Self::State {
        tape.iter().fold(partial,|acc,n| {
            AutomataUTF8::compute(acc,n,linebreak)
        }
        )
    }
}

impl AutomataUTF8 {
    /// Runs the automata over the given tape, generating a partial response
    fn compute(partial: UTF8PartialState, char: &u8,linebreak:char) -> UTF8PartialState {
        // TODO improve performance lol
        let UTF8PartialState(mut expect, mut onword, mut legth, mut stats, mut buff) = partial;
        loop {
            match expect {
                // We are not expecting any character at all. expect and proccess
                // on recursive call instead
                State::New => {
                    expect = State::decode(char);
                }
                State::One => {
                    stats.bytes += 1;
                    buff[0] = *char;

                    // If end we need to add one char to the count (it represents
                    // before we had a char). The program does not count the last
                    // char. Instead, it counts from zero
                    // - Reset buffer to empty
                    // - Write on buff [0]
                    // update stats
                    let asnum = u32::from_le_bytes(buff);
                    let opt_character = char::from_u32(asnum);

                    match opt_character {
                        Some(x) if x == linebreak => {
                            stats.characters += 1;
                            stats.lines += 1;
                            stats.legth = max(stats.legth,legth);
                            legth = 0;
                            if onword {
                                stats.words += 1;
                            }
                            onword = false;
                        },
                        Some(x) => {
                            stats.characters += 1;
                            legth+=1;
                            if isspace!(x as u32) {
                                if onword {
                                    stats.words += 1;
                                    onword = false;
                                }
                            } else {
                                onword = true;
                            }
                        },
                        None => onword = false,
                    }
                    buff.fill(0);
                    expect = State::New;

                    return UTF8PartialState(expect, onword,legth, stats, buff);
                }
                State::Two => {
                    stats.bytes += 1;
                    buff[1] = *char;
                    expect = State::One;
                    return UTF8PartialState(expect, onword,legth, stats, buff);
                }
                State::Three => {
                    stats.bytes += 1;
                    buff[2] = *char;
                    expect = State::Two;
                    return UTF8PartialState(expect, onword,legth, stats, buff);
                }
                State::Four => {
                    stats.bytes += 1;
                    buff[3] = *char;
                    expect = State::Three;
                    return UTF8PartialState(expect, onword,legth, stats, buff);
                }
            }
        }
    }
}
#[cfg(test)]
mod test {
    use std::fs::File;
    use std::io::BufReader;

    use crate::stats::automata::automata_utf8::AutomataUTF8;
    use crate::stats::automata::trait_automata::Automata;
    use crate::stats::stats::Stats;

    fn proccess_file_test(f: &str) -> Stats {
        let reader = BufReader::new(File::open(f).unwrap());
        let stats = AutomataUTF8.stats_from_bufread(Box::new(reader),'\n')
        .unwrap();

        stats
    }

    #[test]
    fn gabriel() {
        let out = proccess_file_test("tests/resources/Gabriel.txt");
        let expected = Stats::new(57, 187, 2694, 2700,580);
        assert_eq!(out, expected)
    }

    #[test]
    fn lorem() {
        let out = proccess_file_test("tests/resources/Lorem_big.txt");
        let expected = Stats::new(1996, 111618, 751539, 751539,1142);
        assert_eq!(out, expected)
    }
    #[test]
    #[ignore]
    fn bible() {
        let out = proccess_file_test("tests/resources/bible.txt");
        let expected = Stats::new(100182, 824036, 4451368, 4451368,78);
        assert_eq!(out, expected)
    }
    #[test]
    fn s1() {
        let out = proccess_file_test("tests/resources/sample1.txt");
        let expected = Stats::new(3, 88, 607, 607,346);
        assert_eq!(out, expected)
    }

    #[test]
    fn s2() {
        let out = proccess_file_test("tests/resources/sample2.txt");
        let expected = Stats::new(12, 423, 2859, 2859,635);
        assert_eq!(out, expected)
    }
    #[test]
    fn s3() {
        let out = proccess_file_test("tests/resources/sample3.txt");
        let expected = Stats::new(20, 546, 3541, 3541,818);
        assert_eq!(out, expected)
    }
    #[test]
    fn small() {
        let out = proccess_file_test("tests/resources/small.txt");
        let expected = Stats::new(1, 3, 18, 18,17);
        assert_eq!(out, expected)
    }
    #[test]
    fn empty() {
        let out = proccess_file_test("tests/resources/empty.txt");
        let expected = Stats::new(0, 0, 0, 0,0);
        assert_eq!(out, expected)
    }

    // TODO this test is weird AF
    #[test]
    #[ignore]
    fn arabic() {
        todo!(); // Legth isn't 0
        let out = proccess_file_test("tests/resources/arabic.txt");
        let expected = Stats::new(0, 10, 58, 105,0);
        assert_eq!(out, expected)
    }
    #[test]
    fn spanish() {
        let out = proccess_file_test("tests/resources/spanish.txt");
        let expected = Stats::new(1, 3, 19, 22,18);
        assert_eq!(out, expected)
    }

    #[test]
    fn french() {
        let out = proccess_file_test("tests/resources/french.txt");
        let expected = Stats::new(0, 10, 57, 61,57);
        assert_eq!(out, expected)
    }
}
