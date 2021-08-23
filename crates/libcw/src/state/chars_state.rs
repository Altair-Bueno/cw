use crate::state::traits::{compute::Compute, partial_state::PartialState};

#[derive(Copy, Clone, Debug,Default)]
pub struct CharState {
    expect: usize,
    num_chars: usize,
}

impl CharState {
    pub fn new() -> CharState {
        Default::default()
    }

    fn compute(self,tape:&[u8],decoder:fn(u8) -> usize) -> CharState{
        tape.iter().fold(self, |acc, n| {
            let (expect, num_chars) = if acc.expect != 0 {
                (acc.expect - 1, acc.num_chars)
            } else {
                ((decoder)(*n), acc.num_chars + 1)
            };
            CharState { expect, num_chars,..self }
        })
    }

    fn utf16_decoder(n: u8) -> usize {
        let shift = n >> 2;
        let mask: u8 = 0b00110110;
        if shift == mask {
            3
        } else {
            1
        }
    }

    fn utf8_decoder(n: u8) -> usize {
        let three: u8 = 0b11110000;
        let two: u8 = 0b11100000;
        let one: u8 = 0b11000000;
        if n & three == three {
            // 11110uuu 10uuzzzz 10yyyyyy 10xxxxxx
            3
        } else if n & two == two {
            // 1110zzzz 10yyyyyy 10xxxxxx
            2
        } else if n & one == one {
            // 110yyyyy 10xxxxxx
            1
        } else {
            0
        }
    }
}
impl PartialState for CharState {
    type Output = usize;

    fn output(&self) -> Self::Output {
        self.num_chars
    }
}
impl Compute for CharState {
    fn utf8_compute(self, tape: &[u8]) -> Self {
        self.compute(tape,CharState::utf8_decoder)
    }

    fn utf16_compute(self, tape: &[u8]) -> Self {
        self.compute(tape,CharState::utf16_decoder)
    }
}

#[cfg(test)]
mod test {
    mod utf16 {
        use crate::state::chars_state::CharState;
        use crate::config::Encoding;
        use crate::state::traits::compute::Compute;
        use crate::state::traits::partial_state::PartialState;

        #[test]
        pub fn test1() {
            let s:Vec<u8> = "hello world".encode_utf16().flat_map(|x| x.to_be_bytes()).collect();
            let out = CharState::new().utf16_compute(s.as_slice()).output();
            assert_eq!(out, 11)
        }

        #[test]
        pub fn test2() {
            let s:Vec<u8> = "".encode_utf16().flat_map(|x| x.to_be_bytes()).collect();
            let out = CharState::new().utf16_compute(s.as_slice()).output();
            assert_eq!(out, 0)
        }

        #[test]
        pub fn test3() {
            let s:Vec<u8> = "a".encode_utf16().flat_map(|x| x.to_be_bytes()).collect();
            let out = CharState::new().utf16_compute(s.as_slice()).output();
            assert_eq!(out, 1)
        }

        #[test]
        pub fn test4() {
            let s:Vec<u8> = "as".encode_utf16().flat_map(|x| x.to_be_bytes()).collect();
            let out = CharState::new().utf16_compute(s.as_slice()).output();
            assert_eq!(out, 2)
        }

        #[test]
        pub fn test5() {
            let s:Vec<u8> = "asfasfweefa sdf asfas".encode_utf16().flat_map(|x| x.to_be_bytes()).collect();
            let out = CharState::new().utf16_compute(s.as_slice()).output();
            assert_eq!(out, 21)
        }

        #[test]
        pub fn test6() {
            let s:Vec<u8> = "ñ".encode_utf16().flat_map(|x| x.to_be_bytes()).collect();
            let out = CharState::new().utf16_compute(s.as_slice()).output();
            assert_eq!(out, 1)
        }

        #[test]
        pub fn test7() {
            let s:Vec<u8> = "ó".encode_utf16().flat_map(|x| x.to_be_bytes()).collect();
            let out = CharState::new().utf16_compute(s.as_slice()).output();
            assert_eq!(out, 1)
        }

        #[test]
        pub fn test8() {
            let out = CharState::new()
                .utf16_compute("ó".encode_utf16().flat_map(|x| x.to_be_bytes()).collect::<Vec<u8>>().as_slice())
                .utf16_compute("ñ".encode_utf16().flat_map(|x| x.to_be_bytes()).collect::<Vec<u8>>().as_slice())
                .utf16_compute("assdfas".encode_utf16().flat_map(|x| x.to_be_bytes()).collect::<Vec<u8>>().as_slice())
                .output();
            assert_eq!(out, 9)
        }
    }
    mod utf8 {
        use std::fs::File;
        use std::io::{BufReader, Read};

        use crate::state::chars_state::CharState;
        use crate::state::traits::{compute::Compute, partial_state::PartialState};
        use crate::config::Encoding;

        #[test]
        pub fn test1() {
            let s = "hello world".as_bytes();
            let out = CharState::new().utf8_compute(s).output();
            assert_eq!(out, 11)
        }

        #[test]
        pub fn test2() {
            let s = "".as_bytes();
            let out = CharState::new().utf8_compute(s).output();
            assert_eq!(out, 0)
        }

        #[test]
        pub fn test3() {
            let s = "a".as_bytes();
            let out = CharState::new().utf8_compute(s).output();
            assert_eq!(out, 1)
        }

        #[test]
        pub fn test4() {
            let s = "as".as_bytes();
            let out = CharState::new().utf8_compute(s).output();
            assert_eq!(out, 2)
        }

        #[test]
        pub fn test5() {
            let s = "asfasfweefa sdf asfas".as_bytes();
            let out = CharState::new().utf8_compute(s).output();
            assert_eq!(out, 21)
        }

        #[test]
        pub fn test6() {
            let s = "ñ".as_bytes();
            let out = CharState::new().utf8_compute(s).output();
            assert_eq!(out, 1)
        }

        #[test]
        pub fn test7() {
            let s = "ó".as_bytes();
            let out = CharState::new().utf8_compute(s).output();
            assert_eq!(out, 1)
        }

        #[test]
        pub fn test8() {
            let out = CharState::new()
                .utf8_compute("ó".as_bytes())
                .utf8_compute("ñ".as_bytes())
                .utf8_compute("assdfas".as_bytes())
                .output();
            assert_eq!(out, 9)
        }

        // Test on files
        fn proccess_file_test(f: &str) -> usize {
            let mut reader = BufReader::new(File::open(f).unwrap());

            let mut state = CharState::new();
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
            let expected = 2694;
            assert_eq!(out, expected)
        }

        #[test]
        fn lorem() {
            let out = proccess_file_test("resources/utf8/Lorem_big.txt");
            assert_eq!(out, 751539)
        }

        #[test]
        fn s1() {
            let out = proccess_file_test("resources/utf8/sample1.txt");
            assert_eq!(out, 607)
        }

        #[test]
        fn s2() {
            let out = proccess_file_test("resources/utf8/sample2.txt");
            assert_eq!(out, 2859)
        }

        #[test]
        fn s3() {
            let out = proccess_file_test("resources/utf8/sample3.txt");
            assert_eq!(out, 3541)
        }

        #[test]
        fn small() {
            let out = proccess_file_test("resources/utf8/small.txt");
            assert_eq!(out, 18)
        }

        #[test]
        fn empty() {
            let out = proccess_file_test("resources/utf8/empty.txt");
            assert_eq!(out, 0)
        }

        #[test]
        fn arabic() {
            // - Legth isn't 0
            // - test weird
            let out = proccess_file_test("resources/utf8/arabic.txt");
            let expected = 58;
            assert_eq!(out, expected)
        }

        #[test]
        fn spanish() {
            let out = proccess_file_test("resources/utf8/spanish.txt");
            let expected = 19;
            assert_eq!(out, expected)
        }

        #[test]
        fn french() {
            let out = proccess_file_test("resources/utf8/french.txt");
            assert_eq!(out, 58)
        }
    }
}
