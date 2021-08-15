use crate::cw_lib::state::traits::{PartialState, Compute};
use regex::bytes::Regex;
use regex::bytes::RegexSet;

// fixme : utf8 support


#[derive(Copy, Clone)]
pub enum Expect {
    New,
    One,
    Two,
    Three,
    Four
}
impl Default for Expect {
    fn default() -> Self {
        Expect::New
    }
}

// TODO
#[derive(Default,Copy, Clone)]
pub struct CharState{
    expect: Expect,
    num_chars:u32
}

impl CharState {
    pub fn new()-> CharState{
        Default::default()
    }
}
impl PartialState for CharState {
    type Output = u32;

    fn output(&self)->Self::Output {
        self.num_chars
    }
}
impl Compute for CharState {
    fn compute(mut self, tape: &[u8]) -> Self {
        if tape.is_empty() {
            return self
        }
        // todo comerle por delante a la cinta
        // Todo stuck on tests
/*
        let four = Regex::new(r"\b11110000...").unwrap();
        let triple = Regex::new(r"\b11100000..").unwrap();
        let double = Regex::new(r"\b11000000.").unwrap();
        let single = Regex::new(r".").unwrap();
        */
        let reg = Regex::new(r"(\xF0...|\xE0..|\xC0.|.)").unwrap();
        let num = reg.captures_iter(tape).count();

        CharState {
            expect: Default::default(),
            num_chars: self.num_chars + num as u32
        }
    }
}

#[cfg(test)]
mod test {
    use crate::cw_lib::state::chars_state::CharState;
    use crate::cw_lib::state::traits::{Compute, PartialState};
    use std::io::{BufReader, Read};
    use std::fs::File;

    #[test]
    pub fn test1() {
        let s = "hello world".as_bytes();
        let out = CharState::new().compute(s).output();
        assert_eq!(out,11)
    }

    // Test on files
    fn proccess_file_test(f: &str) -> u32 {
        let mut reader = BufReader::new(File::open(f).unwrap());

        let mut state = CharState::new();
        let mut buff = [0; 1024];
        loop {
            let read = reader.read(&mut buff).unwrap();
            if read == 0 {
                return state.output();
            }
            state = state.compute(&buff[0..read]);
        }
    }

    #[test]
    fn gabriel() {
        let out = proccess_file_test("tests/resources/Gabriel.txt");
        let expected = 2694;
        assert_eq!(out, expected)
    }

    #[test]
    fn lorem() {
        let out = proccess_file_test("tests/resources/Lorem_big.txt");
        assert_eq!(out, 751539)
    }
    #[test]
    fn bible() {
        let out = proccess_file_test("tests/resources/bible.txt");
        assert_eq!(out, 4451368)
    }
    #[test]
    fn s1() {
        let out = proccess_file_test("tests/resources/sample1.txt");
        assert_eq!(out, 607)
    }

    #[test]
    fn s2() {
        let out = proccess_file_test("tests/resources/sample2.txt");
        assert_eq!(out, 2859)
    }
    #[test]
    fn s3() {
        let out = proccess_file_test("tests/resources/sample3.txt");
        assert_eq!(out, 3541)
    }
    #[test]
    fn small() {
        let out = proccess_file_test("tests/resources/small.txt");
        assert_eq!(out, 18)
    }
    #[test]
    fn empty() {
        let out = proccess_file_test("tests/resources/empty.txt");
        assert_eq!(out, 0)
    }

    #[test]
    fn arabic() {
        // - Legth isn't 0
        // - test weird
        let out = proccess_file_test("tests/resources/arabic.txt");
        let expected = 58;
        assert_eq!(out, expected)
    }
    #[test]
    fn spanish() {
        let out = proccess_file_test("tests/resources/spanish.txt");
        let expected = 19;
        assert_eq!(out, expected)
    }

    #[test]
    fn french() {
        let out = proccess_file_test("tests/resources/french.txt");
        assert_eq!(out, 58)
    }
}