use crate::cw_lib::state::traits::{Compute, PartialState};
use lazy_static::lazy_static;
use regex::bytes::Regex;
// Avoid compiling the regex multiple times inside a loop. Regex should match
// whitespaces as defined by POSIX standard

lazy_static! {
    static ref SEPARATOR_REGEX: Regex = Regex::new(r"(?-u)[\x09\x20\x0A-\x0D]+").unwrap();
}

// Number of words
#[derive(Default, Debug, Copy, Clone)]
pub struct WordsState {
    wordcount: usize,
    onword: bool,
}

impl WordsState {
    pub fn new() -> Self {
        Default::default()
    }
}

impl PartialState for WordsState {
    type Output = usize;
    fn output(&self) -> Self::Output {
        let remaining = if self.onword { 1 } else { 0 };
        self.wordcount + remaining
    }
}

impl Compute for WordsState {
    fn compute(self, tape: &[u8]) -> Self {
        let isseparator = |x: u8| match x {
            0x09 | 0x20 => true,
            x => (0x0A..=0x0D).contains(&x),
        };

        let count = SEPARATOR_REGEX.find_iter(tape).count();

        let count = match tape.get(0) {
            Some(x) if isseparator(*x) && !self.onword => count - 1,
            _ => count,
        };
        let onword = match tape.last() {
            // if last char is separator, we are no longer inside a word
            Some(x) => !isseparator(*x),
            None => self.onword,
        };

        WordsState {
            wordcount: count + self.wordcount,
            onword,
        }
    }
}
#[cfg(test)]
mod test {
    use crate::cw_lib::state::traits::{Compute, PartialState};
    use crate::cw_lib::state::words_state::WordsState;
    use std::fs::File;
    use std::io::{BufReader, Read};

    #[test]
    pub fn test1() {
        let line = "".as_bytes();
        let out = WordsState::new().compute(line).output();
        assert_eq!(out, 0)
    }
    #[test]
    pub fn test2() {
        let line = "hello".as_bytes();
        let out = WordsState::new().compute(line).output();
        assert_eq!(out, 1)
    }
    #[test]
    pub fn test3() {
        let line = "hello world".as_bytes();
        let out = WordsState::new().compute(line).output();
        assert_eq!(out, 2)
    }
    #[test]
    pub fn test4() {
        let line = "hello\nworld".as_bytes();
        let out = WordsState::new().compute(line).output();
        assert_eq!(out, 2)
    }
    #[test]
    pub fn test5() {
        let line = "\nworld".as_bytes();
        let out = WordsState::new().compute(line).output();
        assert_eq!(out, 1)
    }
    #[test]
    pub fn test6() {
        let line = "\n\nworld".as_bytes();
        let out = WordsState::new().compute(line).output();
        assert_eq!(out, 1)
    }
    #[test]
    pub fn test7() {
        let line = "hello\n\n".as_bytes();
        let out = WordsState::new().compute(line).output();
        assert_eq!(out, 1)
    }
    #[test]
    pub fn test8() {
        let line = "texto en español de prueba con número de palabras".as_bytes();
        let out = WordsState::new().compute(line).output();
        assert_eq!(out, 9)
    }
    #[test]
    pub fn test9() {
        let line = "    \t   texto en      español de    prueba    con número\n\t \t de\n palabras"
            .as_bytes();
        let out = WordsState::new().compute(line).output();
        assert_eq!(out, 9)
    }
    #[test]
    pub fn test10() {
        let out = WordsState::new()
            .compute("hell".as_bytes())
            .compute("o ".as_bytes())
            .compute("world".as_bytes())
            .output();
        assert_eq!(out, 2)
    }

    // Test on files
    fn proccess_file_test(f: &str) -> usize {
        let mut reader = BufReader::new(File::open(f).unwrap());

        let mut state = WordsState::new();
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
        let expected = 187;
        assert_eq!(out, expected)
    }

    #[test]
    fn lorem() {
        let out = proccess_file_test("tests/resources/Lorem_big.txt");
        assert_eq!(out, 111618)
    }
    #[test]
    fn bible() {
        let out = proccess_file_test("tests/resources/bible.txt");
        assert_eq!(out, 824036)
    }
    #[test]
    fn s1() {
        let out = proccess_file_test("tests/resources/sample1.txt");
        assert_eq!(out, 88)
    }

    #[test]
    fn s2() {
        let out = proccess_file_test("tests/resources/sample2.txt");
        assert_eq!(out, 423)
    }
    #[test]
    fn s3() {
        let out = proccess_file_test("tests/resources/sample3.txt");
        assert_eq!(out, 546)
    }
    #[test]
    fn small() {
        let out = proccess_file_test("tests/resources/small.txt");
        assert_eq!(out, 3)
    }
    #[test]
    fn empty() {
        let out = proccess_file_test("tests/resources/empty.txt");
        assert_eq!(out, 0)
    }

    /*
    #[test]
    #[ignore]
    fn arabic() {
        // - Legth isn't 0
        // - test weird
        let out = proccess_file_test("tests/resources/arabic.txt");
        let expected = 0;
        assert_eq!(out, expected)
    }
    */
    #[test]
    fn spanish() {
        let out = proccess_file_test("tests/resources/spanish.txt");
        let expected = 3;
        assert_eq!(out, expected)
    }

    #[test]
    fn french() {
        let out = proccess_file_test("tests/resources/french.txt");
        assert_eq!(out, 10)
    }
}
