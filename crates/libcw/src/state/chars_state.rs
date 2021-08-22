use crate::state::traits::{compute::Compute, partial_state::PartialState};
use crate::config::Encoding;

#[derive(Copy, Clone, Debug)]
pub struct CharState {
    expect: usize,
    num_chars: usize,
    fn_tail_length: fn(u8) ->usize,
}

impl CharState {
    pub fn new(encoding:Encoding) -> CharState {
        let fn_tail_length = match encoding {
            Encoding::UTF8 => CharState::utf8,
            Encoding::UTF16 => CharState::utf16,
        };
        CharState {
            fn_tail_length,
            ..Default::default()
        }
    }
    fn utf16(n: u8) -> usize {
        let shift = n >> 2;
        let mask: u8 = 0b00110110;
        if shift == mask {
            3
        } else {
            1
        }
    }

    fn utf8(n: u8) -> usize {
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
impl Default for CharState {
    fn default() -> Self {
        CharState{
            expect: 0,
            num_chars: 0,
            fn_tail_length: CharState::utf8
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
    fn compute(self, tape: &[u8]) -> Self {
        tape.iter().fold(self, |acc, n| {
            let (expect, num_chars) = if acc.expect != 0 {
                (acc.expect - 1, acc.num_chars)
            } else {
                ((acc.fn_tail_length)(*n), acc.num_chars + 1)
            };
            CharState { expect, num_chars,..self }
        })
    }
}

#[cfg(test)]
mod test {
    mod utf16 {
        use crate::state::chars_state::CharState;
        use crate::config::Encoding;
        use crate::state::traits::compute::Compute;
        use crate::state::traits::partial_state::PartialState;
        use std::io::{BufReader, Read};
        use std::fs::File;

        #[test]
        pub fn test1() {
            let s:Vec<u8> = "hello world".encode_utf16().flat_map(|x| x.to_ne_bytes()).collect();
            let out = CharState::new(Encoding::UTF16).compute(s.as_slice()).output();
            assert_eq!(out, 11)
        }

        #[test]
        pub fn test2() {
            let s:Vec<u8> = "".encode_utf16().flat_map(|x| x.to_ne_bytes()).collect();
            let out = CharState::new(Encoding::UTF16).compute(s.as_slice()).output();
            assert_eq!(out, 0)
        }

        #[test]
        pub fn test3() {
            let s:Vec<u8> = "a".encode_utf16().flat_map(|x| x.to_ne_bytes()).collect();
            let out = CharState::new(Encoding::UTF16).compute(s.as_slice()).output();
            assert_eq!(out, 1)
        }

        #[test]
        pub fn test4() {
            let s:Vec<u8> = "as".encode_utf16().flat_map(|x| x.to_ne_bytes()).collect();
            let out = CharState::new(Encoding::UTF16).compute(s.as_slice()).output();
            assert_eq!(out, 2)
        }

        #[test]
        pub fn test5() {
            let s:Vec<u8> = "asfasfweefa sdf asfas".encode_utf16().flat_map(|x| x.to_ne_bytes()).collect();
            let out = CharState::new(Encoding::UTF16).compute(s.as_slice()).output();
            assert_eq!(out, 21)
        }

        #[test]
        pub fn test6() {
            let s:Vec<u8> = "ñ".encode_utf16().flat_map(|x| x.to_ne_bytes()).collect();
            let out = CharState::new(Encoding::UTF16).compute(s.as_slice()).output();
            assert_eq!(out, 1)
        }

        #[test]
        pub fn test7() {
            let s:Vec<u8> = "ó".encode_utf16().flat_map(|x| x.to_ne_bytes()).collect();
            let out = CharState::new(Encoding::UTF16).compute(s.as_slice()).output();
            assert_eq!(out, 1)
        }

        #[test]
        pub fn test8() {
            let out = CharState::new(Encoding::UTF16)
                .compute("ó".encode_utf16().flat_map(|x| x.to_ne_bytes()).collect::<Vec<u8>>().as_slice())
                .compute("ñ".encode_utf16().flat_map(|x| x.to_ne_bytes()).collect::<Vec<u8>>().as_slice())
                .compute("assdfas".encode_utf16().flat_map(|x| x.to_ne_bytes()).collect::<Vec<u8>>().as_slice())
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
            let out = CharState::new(Encoding::UTF8).compute(s).output();
            assert_eq!(out, 11)
        }

        #[test]
        pub fn test2() {
            let s = "".as_bytes();
            let out = CharState::new(Encoding::UTF8).compute(s).output();
            assert_eq!(out, 0)
        }

        #[test]
        pub fn test3() {
            let s = "a".as_bytes();
            let out = CharState::new(Encoding::UTF8).compute(s).output();
            assert_eq!(out, 1)
        }

        #[test]
        pub fn test4() {
            let s = "as".as_bytes();
            let out = CharState::new(Encoding::UTF8).compute(s).output();
            assert_eq!(out, 2)
        }

        #[test]
        pub fn test5() {
            let s = "asfasfweefa sdf asfas".as_bytes();
            let out = CharState::new(Encoding::UTF8).compute(s).output();
            assert_eq!(out, 21)
        }

        #[test]
        pub fn test6() {
            let s = "ñ".as_bytes();
            let out = CharState::new(Encoding::UTF8).compute(s).output();
            assert_eq!(out, 1)
        }

        #[test]
        pub fn test7() {
            let s = "ó".as_bytes();
            let out = CharState::new(Encoding::UTF8).compute(s).output();
            assert_eq!(out, 1)
        }

        #[test]
        pub fn test8() {
            let out = CharState::new(Encoding::UTF8)
                .compute("ó".as_bytes())
                .compute("ñ".as_bytes())
                .compute("assdfas".as_bytes())
                .output();
            assert_eq!(out, 9)
        }

        // Test on files
        fn proccess_file_test(f: &str) -> usize {
            let mut reader = BufReader::new(File::open(f).unwrap());

            let mut state = CharState::new(Encoding::UTF8);
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
        #[ignore]
        fn world() {
            let out = proccess_file_test("resources/utf8/world192.txt");
            assert_eq!(out, 2473400)
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
