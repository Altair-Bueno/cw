use crate::cw::automaton::trait_automaton::Automata;
use crate::cw::automaton::trait_partial_state::PartialState;
use crate::cw::automaton::{OnWord, isspace};
use crate::cw::stats::Stats;
use std::cmp::max;

// UTF8 encoded char uses 4 bytes at most
type UTFClass = Expect;
type CurrentLength = u32;

// What the UTF8 automaton expects next
#[derive(Copy, Clone)]
enum Expect {
    New,
    One,
    Two,
    Three,
    Four,
}
impl Default for Expect {
    fn default() -> Self {
        Expect::New
    }
}

impl Expect {
    pub fn decode(byte: u8) -> Expect {
        let four = 0b11110000;  // 11110uuu 10uuzzzz 10yyyyyy 10xxxxxx
        let three = 0b11100000; // 1110zzzz 10yyyyyy 10xxxxxx
        let two = 0b11000000;   // 110yyyyy 10xxxxxx
        // one (any)                // xxxxxxxx

        if byte & four == four {
            Expect::Four
        } else if byte & three == three {
            Expect::Three
        } else if byte & two == two {
            Expect::Two
        } else {
            Expect::One
        }
    }
}

/// Represents progress for a finite automaton. Can be converted into a final
/// result by using the `result()` function
#[derive(Default)]
pub struct UTF8PartialState(Expect, OnWord, CurrentLength, Stats, UTFClass);

impl PartialState for UTF8PartialState {
    /// Transforms a `UTF8PartialState` into `Stats`
    fn result(self) -> Stats {
        let UTF8PartialState(_, onword, legth, mut stats, _) = self;
        if onword {
            stats.words += 1;
        }
        stats.legth = max(stats.legth, legth);
        stats
    }
}

/// Represents a Finite Deterministic Automata which fetchs it's input from a
/// given tape
pub struct AutomatonUTF8;

impl Automata for AutomatonUTF8 {
    type State = UTF8PartialState;

    /// Runs the automaton over the given tape, generating a partial response
    fn run(&self, partial: Self::State, tape: &[u8], linebreak: u8) -> Self::State {
        tape.iter()
            .fold(partial, |acc, n| AutomatonUTF8::compute(acc, *n, linebreak))
    }
}

impl AutomatonUTF8 {
    /// Transition the automaton's state using the given imput
    fn compute(partial: UTF8PartialState, char: u8, linebreak: u8) -> UTF8PartialState {
        let UTF8PartialState(mut expect, mut onword, mut legth, mut stats, mut class) = partial;
        loop {
            match expect {
                Expect::New => {
                    expect = Expect::decode(char);
                    class = expect;
                }
                Expect::One => {
                    stats.bytes += 1;

                    match (class,char) {
                        (UTFClass::One, x) if x == linebreak => {
                            stats.characters += 1;
                            stats.lines += 1;
                            stats.legth = max(stats.legth, legth);
                            legth = 0;

                            if onword {
                                stats.words += 1;
                                onword = false;
                            }
                        },
                        (UTFClass::One, x) if isspace(x) => {
                            stats.characters += 1;
                            legth += 1;
                            if onword {
                                stats.words += 1;
                                onword = false;
                            }
                        },
                        _ => {
                            stats.characters += 1;
                            legth += 1;
                            onword = true;
                        }
                    }
                    expect = Expect::New;
                    class = expect;

                    return UTF8PartialState(expect, onword, legth, stats, class);
                }
                Expect::Two => {
                    stats.bytes += 1;
                    expect = Expect::One;
                    return UTF8PartialState(expect, onword, legth, stats, class);
                }
                Expect::Three => {
                    stats.bytes += 1;
                    expect = Expect::Two;
                    return UTF8PartialState(expect, onword, legth, stats, class);
                }
                Expect::Four => {
                    stats.bytes += 1;
                    expect = Expect::Three;
                    return UTF8PartialState(expect, onword, legth, stats, class);
                }
            }
        }
    }
}
#[cfg(test)]
mod test {
    use std::fs::File;
    use std::io::BufReader;

    use crate::cw::automaton::automaton_utf8::AutomatonUTF8;
    use crate::cw::automaton::trait_automaton::Automata;
    use crate::cw::stats::Stats;

    fn proccess_file_test(f: &str) -> Stats {
        let reader = BufReader::new(File::open(f).unwrap());
        let stats = AutomatonUTF8.stats_from_bufread(reader, b'\n').unwrap();

        stats
    }

    #[test]
    fn gabriel() {
        let out = proccess_file_test("tests/resources/Gabriel.txt");
        let expected = Stats::new(57, 187, 2694, 2700, 580);
        assert_eq!(out, expected)
    }

    #[test]
    fn lorem() {
        let out = proccess_file_test("tests/resources/Lorem_big.txt");
        let expected = Stats::new(1996, 111618, 751539, 751539, 1142);
        assert_eq!(out, expected)
    }
    #[test]
    #[ignore] // On CI does fail. I don't know why
    fn bible() {
        let out = proccess_file_test("tests/resources/bible.txt");
        let expected = Stats::new(100182, 824036, 4451368, 4451368, 78);
        assert_eq!(out, expected)
    }
    #[test]
    fn s1() {
        let out = proccess_file_test("tests/resources/sample1.txt");
        let expected = Stats::new(3, 88, 607, 607, 346);
        assert_eq!(out, expected)
    }

    #[test]
    fn s2() {
        let out = proccess_file_test("tests/resources/sample2.txt");
        let expected = Stats::new(12, 423, 2859, 2859, 635);
        assert_eq!(out, expected)
    }
    #[test]
    fn s3() {
        let out = proccess_file_test("tests/resources/sample3.txt");
        let expected = Stats::new(20, 546, 3541, 3541, 818);
        assert_eq!(out, expected)
    }
    #[test]
    fn small() {
        let out = proccess_file_test("tests/resources/small.txt");
        let expected = Stats::new(1, 3, 18, 18, 17);
        assert_eq!(out, expected)
    }
    #[test]
    fn empty() {
        let out = proccess_file_test("tests/resources/empty.txt");
        let expected = Stats::new(0, 0, 0, 0, 0);
        assert_eq!(out, expected)
    }

    #[test]
    #[ignore]
    fn arabic() {
        // - Legth isn't 0
        // - test weird
        todo!();
        let out = proccess_file_test("tests/resources/arabic.txt");
        let expected = Stats::new(0, 10, 58, 105, 0);
        assert_eq!(out, expected)
    }
    #[test]
    fn spanish() {
        let out = proccess_file_test("tests/resources/spanish.txt");
        let expected = Stats::new(1, 3, 19, 22, 18);
        assert_eq!(out, expected)
    }

    #[test]
    fn french() {
        let out = proccess_file_test("tests/resources/french.txt");
        let expected = Stats::new(0, 10, 58, 61, 58);
        assert_eq!(out, expected)
    }
}
