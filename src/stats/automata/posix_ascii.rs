use crate::isspace;
use crate::stats::automata::trait_automata::Automata;
use crate::stats::automata::trait_partial_state::PartialState;
use crate::stats::automata::OnWord;
use crate::stats::stats::Stats;

#[cfg(unused)]
mod utils {
    /// Defined on C95: wctype.h
    /// https://en.cppreference.com/w/c/string/wide/iswspace
    pub fn isspace(char: u8) -> bool {
        (char == 0x9) || (char == 0x20) || (char >= 0xA && char <= 0xD)
    }
    /// Defined on C95: wctype.h
    /// https://en.cppreference.com/w/c/string/wide/iswspace
    pub fn isalpha(char: u8) -> bool {
        (char >= 0x41 && char <= 0x5A) || (char >= 0x61 && char <= 0x7A)
    }
}

#[derive(Default)]
pub struct PosixASCIIPartialState(OnWord, Stats);

impl PartialState for PosixASCIIPartialState {
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

    fn run(&self, partial: Self::State, tape: &[u8],linebreak:char) ->
                                                                 Self::State {
        tape.iter().fold(partial,|acc,n| PosixASCII::compute(acc,n,linebreak))
    }
}

impl PosixASCII {
    fn compute(partial: PosixASCIIPartialState, byte: &u8,linebreak:char) ->
                                                            PosixASCIIPartialState {
        let PosixASCIIPartialState(mut onword, mut stats) = partial;
        stats.characters += 1;
        stats.bytes += 1;
        match byte {
            x if *x as char == linebreak => {
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

#[cfg(test)]
mod test {
    use crate::stats::automata::posix_ascii::PosixASCII;
    use crate::stats::automata::trait_automata::Automata;
    use std::fs::File;
    use std::io::BufReader;
    use crate::stats::stats::Stats;

    fn proccess_file_test(f: &str) -> Stats {
        let reader = BufReader::new(File::open(f).unwrap());
        let stats = PosixASCII.stats_from_bufread(Box::new(reader),'\n')
            .unwrap();

        stats
    }

    #[test]
    fn gabriel() {
        let out = proccess_file_test("tests/resources/Gabriel.txt");
        let expected = Stats::new(57, 187, 2700, 2700);
        assert_eq!(out, expected)
    }

    #[test]
    fn lorem() {
        let out = proccess_file_test("tests/resources/Lorem_big.txt");
        let expected = Stats::new(1996, 111618, 751539, 751539);
        assert_eq!(out, expected)
    }
    #[test]
    #[ignore]
    fn bible() {
        let out = proccess_file_test("tests/resources/bible.txt");
        let expected = Stats::new(100182, 824036, 4451368, 4451368);
        assert_eq!(out, expected)
    }
    #[test]
    fn s1() {
        let out = proccess_file_test("tests/resources/sample1.txt");
        let expected = Stats::new(3, 88, 607, 607);
        assert_eq!(out, expected)
    }

    #[test]
    fn s2() {
        let out = proccess_file_test("tests/resources/sample2.txt");
        let expected = Stats::new(12, 423, 2859, 2859);
        assert_eq!(out, expected)
    }
    #[test]
    fn s3() {
        let out = proccess_file_test("tests/resources/sample3.txt");
        let expected = Stats::new(20, 546, 3541, 3541);
        assert_eq!(out, expected)
    }
    #[test]
    fn small() {
        let out = proccess_file_test("tests/resources/small.txt");
        let expected = Stats::new(1, 3, 18, 18);
        assert_eq!(out, expected)
    }
    #[test]
    fn empty() {
        let out = proccess_file_test("tests/resources/empty.txt");
        let expected = Stats::new(0, 0, 0, 0);
        assert_eq!(out, expected)
    }
}
