use crate::cw_lib::state::traits::{PartialState, Compute};
use regex::bytes::Regex;
use lazy_static::lazy_static;

lazy_static!{
    static ref reg:Regex = Regex::new(r"(?us:.)").unwrap();
}


#[derive(Default,Copy, Clone)]
pub struct CharState{
    expect: usize,
    num_chars:usize
}

impl CharState {
    pub fn new()-> CharState{
        Default::default()
    }

    fn eat_from_tape(eat:usize,tape:&[u8]) -> (usize,&[u8]) {
        if tape.len() > eat {
            (0,&tape[eat..])
        } else {
            let left= eat - tape.len();
            (left,&[])
        }
    }
}
impl PartialState for CharState {
    type Output = usize;

    fn output(&self)->Self::Output {
        self.num_chars
    }
}
impl Compute for CharState {
    fn compute(mut self, tape: &[u8]) -> Self {
        let (mut state,tape) = CharState::eat_from_tape(self.expect, tape);
        // run over the rest of the tape

        let (last_match_index,count) = reg
            .find_iter(tape)
            //.inspect(|x| println!("{}",std::str::from_utf8(x.as_bytes()).unwrap()))
            .fold((0,0),|(_,c),n|{
                (n.end(),c+1)
        });

        if tape.len() != 0 {
            let eat_next = tape.len() - last_match_index;
            // We are sure that this is not the end
            state = eat_next;
        }
        CharState {
            expect: state,
            num_chars: self.num_chars + count
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

    #[test]
    pub fn test2() {
        let s = "".as_bytes();
        let out = CharState::new().compute(s).output();
        assert_eq!(out,0)
    }
    #[test]
    pub fn test3() {
        let s = "a".as_bytes();
        let out = CharState::new().compute(s).output();
        assert_eq!(out,1)
    }
    #[test]
    pub fn test4() {
        let s = "as".as_bytes();
        let out = CharState::new().compute(s).output();
        assert_eq!(out,2)
    }
    #[test]
    pub fn test5() {
        let s = "asfasfweefa sdf asfas".as_bytes();
        let out = CharState::new().compute(s).output();
        assert_eq!(out,21)
    }
    #[test]
    pub fn test6() {
        let s = "ñ".as_bytes();
        let out = CharState::new().compute(s).output();
        assert_eq!(out,1)
    }
    #[test]
    pub fn test7() {
        let s = "ó".as_bytes();
        let out = CharState::new().compute(s).output();
        assert_eq!(out,1)
    }
    #[test]
    pub fn test8() {
        let out = CharState::new()
            .compute("ó".as_bytes())
            .compute("ñ".as_bytes())
            .compute("assdfas".as_bytes())
            .output();
        assert_eq!(out,9)
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