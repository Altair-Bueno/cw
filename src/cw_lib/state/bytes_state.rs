use crate::cw_lib::state::traits::{PartialState, Compute};

// fixme Works as intended


/// Number of bytes
#[derive(Default,Debug,Copy, Clone)]
pub struct BytesState {
    bytecount:u32,
}
impl BytesState {
    pub fn new() -> Self {
        Default::default()
    }
}

impl PartialState for BytesState {
    type Output = u32;
    fn output(&self)->Self::Output{
        self.bytecount
    }
}

impl Compute for BytesState {
    fn compute(mut self, tape: &[u8]) -> Self {
        self.bytecount += (tape.len() as u32);
        self
    }
}

#[cfg(test)]
mod test {
    use crate::cw_lib::state::bytes_state::BytesState;
    use crate::cw_lib::state::traits::{Compute, PartialState};
    use std::io::{BufReader, Read};
    use std::fs::File;

    #[test]
    pub fn test1 () {
        let bytes = "hello world".as_bytes();
        let parse = BytesState::new().compute(bytes).output();
        assert_eq!(parse,11)
    }
    #[test]
    pub fn test2 () {
        let bytes = "".as_bytes();
        let parse = BytesState::new().compute(bytes).output();
        assert_eq!(parse,0)
    }
    #[test]
    pub fn test3 () {
        let bytes = "ñ".as_bytes();
        let parse = BytesState::new().compute(bytes).output();
        assert_eq!(parse,2)
    }
    #[test]
    pub fn test4() {
        let parse = BytesState::new()
            .compute("ñ".as_bytes())
            .compute("hello".as_bytes())
            .compute(" ass sa fda fsj fasd ".as_bytes())
            .output();
        assert_eq!(parse,28)
    }

    // Test on files
    fn proccess_file_test(f: &str) -> u32 {
        let mut reader = BufReader::new(File::open(f).unwrap());

        let mut state = BytesState::new();
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
        let expected = 2700;
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
        let expected = 105;
        assert_eq!(out, expected)
    }
    #[test]
    fn spanish() {
        let out = proccess_file_test("tests/resources/spanish.txt");
        let expected = 22;
        assert_eq!(out, expected)
    }

    #[test]
    fn french() {
        let out = proccess_file_test("tests/resources/french.txt");
        assert_eq!(out, 61)
    }

}
