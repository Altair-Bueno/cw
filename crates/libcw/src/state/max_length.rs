use std::cmp::max;

use crate::config::LineBreak;
use crate::state::chars_state::CharState;
use crate::state::traits::{compute::Compute, partial_state::PartialState};

/// Max length
#[derive(Debug, Copy, Clone)]
pub struct MaxLengthState {
    max_length_found: usize,
    //line_count: usize,
    //char_count:usize,
    linebreak: LineBreak,
    char_state: CharState,
}

impl Default for MaxLengthState {
    fn default() -> Self {
        MaxLengthState::new(LineBreak::default())
    }
}

impl MaxLengthState {
    pub fn new(linebreak: LineBreak) -> Self {
        MaxLengthState {
            max_length_found: 0,
            //line_count: 0,
            //char_count: 0,
            linebreak,
            char_state: CharState::new(),
        }
    }
}

impl PartialState for MaxLengthState {
    type Output = usize;
    fn output(&self) -> Self::Output {
        let char_state_output = self.char_state.output();
        max(self.max_length_found, char_state_output)
        // let line_count = self.line_count;
        // let character_count = char_state_output + self.char_count;
    }
}

impl Compute for MaxLengthState {
    fn utf8_compute(self, tape: &[u8]) -> Self {
        let b = self.linebreak.get_separator();
        tape.split_inclusive(|x| *x == b).fold(self, |state, next| {
            let on_line = next.last().map(|x| *x != b).unwrap_or(true);
            let count_chars_state = state.char_state.utf8_compute(next);
            // Count lines if it's the end of the line. Update character
            // count in the end
            if on_line {
                // No linebreak detected. Still same line as before
                MaxLengthState {
                    char_state: count_chars_state,
                    ..state
                }
            } else {
                let count = count_chars_state.output();
                MaxLengthState {
                    max_length_found: max(count - 1, state.max_length_found),
                    //line_count: state.line_count + 1,
                    //char_count: state.char_count + count,
                    char_state: CharState::new(),
                    ..state
                }
            }
        })
    }

    fn utf16_compute(self, tape: &[u8]) -> Self {
        let b = self.linebreak.get_separator();
        tape.split_inclusive(|x| *x == b).fold(self, |state, next| {
            let on_line = next.last().map(|x| *x != b).unwrap_or(true);
            let count_chars_state = state.char_state.utf16_compute(next);
            // Count lines if it's the end of the line. Update character
            // count in the end
            if on_line {
                // No linebreak detected. Still same line as before
                MaxLengthState {
                    char_state: count_chars_state,
                    ..state
                }
            } else {
                let count = count_chars_state.output();
                MaxLengthState {
                    max_length_found: max(count - 1, state.max_length_found),
                    //line_count: state.line_count + 1,
                    //char_count: state.char_count + count,
                    char_state: CharState::new(),
                    ..state
                }
            }
        })
    }
}

#[cfg(test)]
mod test {
    mod utf16 {
        use crate::config::LineBreak;
        use crate::state::max_length::MaxLengthState;
        use crate::state::traits::compute::Compute;
        use crate::state::traits::partial_state::PartialState;

        #[test]
        pub fn test1() {
            let line = ""
                .encode_utf16()
                .flat_map(u16::to_be_bytes)
                .collect::<Vec<u8>>();
            let out = MaxLengthState::new(LineBreak::LF)
                .utf16_compute(line.as_slice())
                .output();
            assert_eq!(out, 0)
        }

        #[test]
        pub fn test2() {
            let line = "hello\n"
                .encode_utf16()
                .flat_map(u16::to_be_bytes)
                .collect::<Vec<u8>>();
            let out = MaxLengthState::new(LineBreak::LF)
                .utf16_compute(line.as_slice())
                .output();
            assert_eq!(out, 5)
        }

        #[test]
        pub fn test3() {
            let line = "hello\nworld"
                .encode_utf16()
                .flat_map(u16::to_be_bytes)
                .collect::<Vec<u8>>();
            let out = MaxLengthState::new(LineBreak::LF)
                .utf16_compute(line.as_slice())
                .output();
            assert_eq!(out, 5)
        }

        #[test]
        pub fn test4() {
            let line = "hello\nworldjsafs\n"
                .encode_utf16()
                .flat_map(u16::to_be_bytes)
                .collect::<Vec<u8>>();
            let out = MaxLengthState::new(LineBreak::LF)
                .utf16_compute(line.as_slice())
                .output();
            assert_eq!(out, 10)
        }

        #[test]
        pub fn test5() {
            let line = "hello\nworldjsafs\nshjksafhjkasfjhkfajshdjhksdfa"
                .encode_utf16()
                .flat_map(u16::to_be_bytes)
                .collect::<Vec<u8>>();
            let out = MaxLengthState::new(LineBreak::LF)
                .utf16_compute(line.as_slice())
                .output();
            assert_eq!(out, 29)
        }

        #[test]
        pub fn test6() {
            let s1 = "hskjaskl a jadsjfjsdjk a asda dsfksa ."
                .encode_utf16()
                .flat_map(u16::to_be_bytes)
                .collect::<Vec<u8>>();
            let s2 = "jkhsajkjafsdjkafsjkafsd"
                .encode_utf16()
                .flat_map(u16::to_be_bytes)
                .collect::<Vec<u8>>();
            let s3 = "iassfdaafsd\n"
                .encode_utf16()
                .flat_map(u16::to_be_bytes)
                .collect::<Vec<u8>>();
            let s4 = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum."
                .encode_utf16().flat_map(u16::to_be_bytes).collect::<Vec<u8>>();
            let out = MaxLengthState::new(LineBreak::LF)
                .utf16_compute(s1.as_slice())
                .utf16_compute(s2.as_slice())
                .utf16_compute(s3.as_slice())
                .utf16_compute(s4.as_slice())
                .output();
            assert_eq!(out, 445)
        }
    }

    mod utf8 {
        use std::fs::File;
        use std::io::{BufReader, Read};

        use crate::config::LineBreak;
        use crate::state::max_length::MaxLengthState;
        use crate::state::traits::{compute::Compute, partial_state::PartialState};

        #[test]
        pub fn test1() {
            let line = "".as_bytes();
            let out = MaxLengthState::new(LineBreak::LF)
                .utf8_compute(line)
                .output();
            assert_eq!(out, 0)
        }

        #[test]
        pub fn test2() {
            let line = "hello\n".as_bytes();
            let out = MaxLengthState::new(LineBreak::LF)
                .utf8_compute(line)
                .output();
            assert_eq!(out, 5)
        }

        #[test]
        pub fn test3() {
            let line = "hello\nworld".as_bytes();
            let out = MaxLengthState::new(LineBreak::LF)
                .utf8_compute(line)
                .output();
            assert_eq!(out, 5)
        }

        #[test]
        pub fn test4() {
            let line = "hello\nworldjsafs\n".as_bytes();
            let out = MaxLengthState::new(LineBreak::LF)
                .utf8_compute(line)
                .output();
            assert_eq!(out, 10)
        }

        #[test]
        pub fn test5() {
            let line = "hello\nworldjsafs\nshjksafhjkasfjhkfajshdjhksdfa".as_bytes();
            let out = MaxLengthState::new(LineBreak::LF)
                .utf8_compute(line)
                .output();
            assert_eq!(out, 29)
        }

        #[test]
        pub fn test6() {
            let out = MaxLengthState::new(LineBreak::LF)
                .utf8_compute("hskjaskl a jadsjfjsdjk a asda dsfksa .".as_bytes())
                .utf8_compute("jkhsajkjafsdjkafsjkafsd".as_bytes())
                .utf8_compute("iassfdaafsd\n".as_bytes())
                .utf8_compute("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.".as_bytes())
                .output();
            assert_eq!(out, 445)
        }

        // Test on files
        fn proccess_file_test(f: &str) -> usize {
            let mut reader = BufReader::new(File::open(f).unwrap());

            let mut state = MaxLengthState::new(LineBreak::LF);
            let mut buff = [0; 1024];
            loop {
                let read = reader.read(&mut buff).unwrap();
                if read == 0 {
                    return state.output();
                }
                state = state.utf8_compute(&buff[0..read]);
            }
        }

        #[test]
        fn gabriel() {
            let out = proccess_file_test("resources/utf8/Gabriel.txt");
            let expected = 580;
            assert_eq!(out, expected)
        }

        #[test]
        fn lorem() {
            let out = proccess_file_test("resources/utf8/Lorem_big.txt");
            assert_eq!(out, 1142)
        }

        #[test]
        #[ignore]
        fn world() {
            let out = proccess_file_test("resources/utf8/world192.txt");
            assert_eq!(out, 81)
        }

        #[test]
        fn s1() {
            let out = proccess_file_test("resources/utf8/sample1.txt");
            assert_eq!(out, 346)
        }

        #[test]
        fn s2() {
            let out = proccess_file_test("resources/utf8/sample2.txt");
            assert_eq!(out, 635)
        }

        #[test]
        fn s3() {
            let out = proccess_file_test("resources/utf8/sample3.txt");
            assert_eq!(out, 818)
        }

        #[test]
        fn small() {
            let out = proccess_file_test("resources/utf8/small.txt");
            assert_eq!(out, 17)
        }

        #[test]
        fn empty() {
            let out = proccess_file_test("resources/utf8/empty.txt");
            assert_eq!(out, 0)
        }

        #[test]
        fn spanish() {
            let out = proccess_file_test("resources/utf8/spanish.txt");
            let expected = 18;
            assert_eq!(out, expected)
        }

        #[test]
        fn french() {
            let out = proccess_file_test("resources/utf8/french.txt");
            assert_eq!(out, 58)
        }
    }
}
