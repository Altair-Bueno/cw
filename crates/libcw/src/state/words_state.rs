use crate::state::traits::{compute::Compute, partial_state::PartialState};

// Number of words
#[derive(Copy, Clone,Default)]
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
    fn utf8_compute(self, tape: &[u8]) -> Self {
        let is_separator = |x: u8| match x {
            0x20 | 0x09 => true,
            x => (0x0A..=0x0D).contains(&x),
        };

        tape.iter().fold(self, |acc, n| {
            let onword = !is_separator(*n);
            let wordcount = acc.wordcount + {
                if acc.onword && !onword {
                    1
                } else {
                    0
                }
            };

            WordsState { wordcount, onword }
        })
    }

    fn utf16_compute(self, tape: &[u8]) -> Self {
        let is_separator = |x: u8| match x {
            0x20 | 0x09 => true,
            x => (0x0A..=0x0D).contains(&x),
        };
        let mut temp = true;
        tape.iter().filter(|_| {
            temp = !temp;
            temp
        }).fold(self, |acc, n| {
            let onword = !is_separator(*n);
            let wordcount = acc.wordcount + {
                if acc.onword && !onword {
                    1
                } else {
                    0
                }
            };

            WordsState { wordcount, onword }
        })
    }
}

#[cfg(test)]
mod test {
    mod utf16 {

        use std::fs::File;
        use std::io::{BufReader, Read};

        use crate::state::traits::{compute::Compute, partial_state::PartialState};
        use crate::state::words_state::WordsState;

        #[test]
        pub fn test1() {
            let line = "".encode_utf16().flat_map(u16::to_be_bytes).collect::<Vec<u8>>();
            let out = WordsState::new().utf8_compute(line.as_slice()).output();
            assert_eq!(out, 0)
        }

        #[test]
        pub fn test2() {
            let line = "hello".encode_utf16().flat_map(u16::to_be_bytes).collect::<Vec<u8>>();
            let out = WordsState::new().utf8_compute(line.as_slice()).output();
            assert_eq!(out, 1)
        }

        #[test]
        pub fn test3() {
            let line = "hello world".encode_utf16().flat_map(u16::to_be_bytes).collect::<Vec<u8>>();
            let out = WordsState::new().utf8_compute(line.as_slice()).output();
            assert_eq!(out, 2)
        }

        #[test]
        pub fn test4() {
            let line = "hello\nworld".encode_utf16().flat_map(u16::to_be_bytes).collect::<Vec<u8>>();
            let out = WordsState::new().utf8_compute(line.as_slice()).output();
            assert_eq!(out, 2)
        }

        #[test]
        pub fn test5() {
            let line = "\nworld".encode_utf16().flat_map(u16::to_be_bytes).collect::<Vec<u8>>();
            let out = WordsState::new().utf8_compute(line.as_slice()).output();
            assert_eq!(out, 1)
        }

        #[test]
        pub fn test6() {
            let line = "\n\nworld".encode_utf16().flat_map(u16::to_be_bytes).collect::<Vec<u8>>();
            let out = WordsState::new().utf8_compute(line.as_slice()).output();
            assert_eq!(out, 1)
        }

        #[test]
        pub fn test7() {
            let line = "hello\n\n".encode_utf16().flat_map(u16::to_be_bytes).collect::<Vec<u8>>();
            let out = WordsState::new().utf8_compute(line.as_slice()).output();
            assert_eq!(out, 1)
        }

        #[test]
        pub fn test8() {
            let line = "texto en español de prueba con número de palabras".encode_utf16().flat_map(u16::to_be_bytes).collect::<Vec<u8>>();
            let out = WordsState::new().utf8_compute(line.as_slice()).output();
            assert_eq!(out, 9)
        }

        #[test]
        pub fn test9() {
            let line = "    \t   texto en      español de    prueba    con número\n\t \t de\n palabras"
                .encode_utf16().flat_map(u16::to_be_bytes).collect::<Vec<u8>>();
            let out = WordsState::new().utf8_compute(line.as_slice()).output();
            assert_eq!(out, 9)
        }

        #[test]
        pub fn test10() {
            let s1 = "hell".encode_utf16().flat_map(u16::to_be_bytes).collect::<Vec<u8>>();
            let s2 ="o ".encode_utf16().flat_map(u16::to_be_bytes).collect::<Vec<u8>>();
            let s3 = "world".encode_utf16().flat_map(u16::to_be_bytes).collect::<Vec<u8>>();
            let out = WordsState::new()
                .utf8_compute(s1.as_slice())
                .utf8_compute(s2.as_slice())
                .utf8_compute(s3.as_slice())
                .output();
            assert_eq!(out, 2)
        }
    }
    mod utf8 {
        use std::fs::File;
        use std::io::{BufReader, Read};

        use crate::state::traits::{compute::Compute, partial_state::PartialState};
        use crate::state::words_state::WordsState;

        #[test]
        pub fn test1() {
            let line = "".as_bytes();
            let out = WordsState::new().utf8_compute(line).output();
            assert_eq!(out, 0)
        }

        #[test]
        pub fn test2() {
            let line = "hello".as_bytes();
            let out = WordsState::new().utf8_compute(line).output();
            assert_eq!(out, 1)
        }

        #[test]
        pub fn test3() {
            let line = "hello world".as_bytes();
            let out = WordsState::new().utf8_compute(line).output();
            assert_eq!(out, 2)
        }

        #[test]
        pub fn test4() {
            let line = "hello\nworld".as_bytes();
            let out = WordsState::new().utf8_compute(line).output();
            assert_eq!(out, 2)
        }

        #[test]
        pub fn test5() {
            let line = "\nworld".as_bytes();
            let out = WordsState::new().utf8_compute(line).output();
            assert_eq!(out, 1)
        }

        #[test]
        pub fn test6() {
            let line = "\n\nworld".as_bytes();
            let out = WordsState::new().utf8_compute(line).output();
            assert_eq!(out, 1)
        }

        #[test]
        pub fn test7() {
            let line = "hello\n\n".as_bytes();
            let out = WordsState::new().utf8_compute(line).output();
            assert_eq!(out, 1)
        }

        #[test]
        pub fn test8() {
            let line = "texto en español de prueba con número de palabras".as_bytes();
            let out = WordsState::new().utf8_compute(line).output();
            assert_eq!(out, 9)
        }

        #[test]
        pub fn test9() {
            let line = "    \t   texto en      español de    prueba    con número\n\t \t de\n palabras"
                .as_bytes();
            let out = WordsState::new().utf8_compute(line).output();
            assert_eq!(out, 9)
        }

        #[test]
        pub fn test10() {
            let out = WordsState::new()
                .utf8_compute("hell".as_bytes())
                .utf8_compute("o ".as_bytes())
                .utf8_compute("world".as_bytes())
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
                state = state.utf8_compute(&buff[0..read]);
            }
        }

        #[test]
        fn gabriel() {
            let out = proccess_file_test("resources/utf8/Gabriel.txt");
            let expected = 187;
            assert_eq!(out, expected)
        }

        #[test]
        fn lorem() {
            let out = proccess_file_test("resources/utf8/Lorem_big.txt");
            assert_eq!(out, 111618)
        }

        #[test]
        #[ignore]
        fn world() {
            let out = proccess_file_test("resources/utf8/world192.txt");
            assert_eq!(out, 326075)
        }

        #[test]
        fn s1() {
            let out = proccess_file_test("resources/utf8/sample1.txt");
            assert_eq!(out, 88)
        }

        #[test]
        fn s2() {
            let out = proccess_file_test("resources/utf8/sample2.txt");
            assert_eq!(out, 423)
        }

        #[test]
        fn s3() {
            let out = proccess_file_test("resources/utf8/sample3.txt");
            assert_eq!(out, 546)
        }

        #[test]
        fn small() {
            let out = proccess_file_test("resources/utf8/small.txt");
            assert_eq!(out, 3)
        }

        #[test]
        fn empty() {
            let out = proccess_file_test("resources/utf8/empty.txt");
            assert_eq!(out, 0)
        }

        /*
    #[test]
    #[ignore]
    fn arabic() {
        // - Legth isn't 0
        // - test weird
        let out = proccess_file_test("resources/utf8/arabic.txt");
        let expected = 0;
        assert_eq!(out, expected)
    }
    */
        #[test]
        fn spanish() {
            let out = proccess_file_test("resources/utf8/spanish.txt");
            let expected = 3;
            assert_eq!(out, expected)
        }

        #[test]
        fn french() {
            let out = proccess_file_test("resources/utf8/french.txt");
            assert_eq!(out, 10)
        }
    }
}
