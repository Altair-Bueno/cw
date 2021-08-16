use crate::cw_lib::state::traits::{Compute, PartialState};
use std::cmp::max;

// fixme: Does not work. Neets utf8 support

/// Max length
#[derive(Debug, Copy, Clone)]
pub struct MaxLengthState {
    buffer: usize,
    champion: usize,
    linebreak: u8,
}
impl Default for MaxLengthState {
    fn default() -> Self {
        MaxLengthState::new(b'\n')
    }
}

impl MaxLengthState {
    pub fn new(linebreak: u8) -> Self {
        MaxLengthState {
            buffer: 0,
            champion: 0,
            linebreak,
        }
    }
}

impl PartialState for MaxLengthState {
    type Output = usize;
    fn output(&self) -> Self::Output {
        max(self.champion, self.buffer)
    }
}

impl Compute for MaxLengthState {
    fn compute(self, tape: &[u8]) -> Self {
        tape.split_inclusive(|x| self.linebreak == *x)
            .map(|x| {
                let mut n_chars = x.len();
                let end = if let Some(x) = x.last() {
                    *x == b'\n'
                } else {
                    false
                };
                if end {
                    n_chars -= 1
                }
                // n_chars: number of chars without \n
                // end: If the line ended with \n or not
                (n_chars, end)
            })
            .fold(self, |_, n| {
                let (this_len, buffer) = if n.1 {
                    (self.buffer + n.0, 0)
                } else {
                    (0, self.buffer + n.0)
                };
                MaxLengthState {
                    buffer,
                    champion: max(this_len, self.champion),
                    ..self
                }
            })
    }
}

#[cfg(test)]
mod test {
    use crate::cw_lib::state::max_length::MaxLengthState;
    use crate::cw_lib::state::traits::{Compute, PartialState};
    use std::fs::File;
    use std::io::{BufReader, Read};

    #[test]
    pub fn test1() {
        let line = "".as_bytes();
        let out = MaxLengthState::new(b'\n').compute(line).output();
        assert_eq!(out, 0)
    }
    #[test]
    pub fn test2() {
        let line = "hello\n".as_bytes();
        let out = MaxLengthState::new(b'\n').compute(line).output();
        assert_eq!(out, 5)
    }
    #[test]
    pub fn test3() {
        let line = "hello\nworld".as_bytes();
        let out = MaxLengthState::new(b'\n').compute(line).output();
        assert_eq!(out, 5)
    }
    #[test]
    pub fn test4() {
        let line = "hello\nworldjsafs\n".as_bytes();
        let out = MaxLengthState::new(b'\n').compute(line).output();
        assert_eq!(out, 10)
    }
    #[test]
    pub fn test5() {
        let line = "hello\nworldjsafs\nshjksafhjkasfjhkfajshdjhksdfa".as_bytes();
        let out = MaxLengthState::new(b'\n').compute(line).output();
        assert_eq!(out, 29)
    }
    #[test]
    pub fn test6() {
        let out = MaxLengthState::new(b'\n')
            .compute("hskjaskl a jadsjfjsdjk a asda dsfksa .".as_bytes())
            .compute("jkhsajkjafsdjkafsjkafsd".as_bytes())
            .compute("iassfdaafsd\n".as_bytes())
            .compute("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.".as_bytes())
            .output();
        assert_eq!(out, 445)
    }

    // Test on files
    fn proccess_file_test(f: &str) -> usize {
        let mut reader = BufReader::new(File::open(f).unwrap());

        let mut state = MaxLengthState::new(b'\n');
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
        let expected = 580;
        assert_eq!(out, expected)
    }

    #[test]
    fn lorem() {
        let out = proccess_file_test("tests/resources/Lorem_big.txt");
        assert_eq!(out, 1142)
    }
    #[test]
    fn bible() {
        let out = proccess_file_test("tests/resources/bible.txt");
        assert_eq!(out, 78)
    }
    #[test]
    fn s1() {
        let out = proccess_file_test("tests/resources/sample1.txt");
        assert_eq!(out, 346)
    }

    #[test]
    fn s2() {
        let out = proccess_file_test("tests/resources/sample2.txt");
        assert_eq!(out, 635)
    }
    #[test]
    fn s3() {
        let out = proccess_file_test("tests/resources/sample3.txt");
        assert_eq!(out, 818)
    }
    #[test]
    fn small() {
        let out = proccess_file_test("tests/resources/small.txt");
        assert_eq!(out, 17)
    }
    #[test]
    fn empty() {
        let out = proccess_file_test("tests/resources/empty.txt");
        assert_eq!(out, 0)
    }

    #[test]
    #[ignore]
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
        let expected = 18;
        assert_eq!(out, expected)
    }

    #[test]
    fn french() {
        let out = proccess_file_test("tests/resources/french.txt");
        assert_eq!(out, 58)
    }
}
