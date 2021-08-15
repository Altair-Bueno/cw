use crate::cw_lib::state::traits::{PartialState, Compute};
/// number of lines
#[derive(Debug,Copy, Clone)]
pub struct LinesState {
    linescount:u32,
    linebreak:u8,
}
impl Default for LinesState {
    fn default() -> Self {
        LinesState::new(b'\n')
    }
}

impl LinesState {
    pub fn new(linebreak:u8) -> Self {
        LinesState{ linescount: 0, linebreak }
    }
}

impl PartialState for LinesState {
    type Output = u32;
    fn output(&self)->Self::Output{
        self.linescount
    }
}
impl Compute for LinesState {
    fn compute(self, tape: &[u8]) -> Self {
        let line_breaks = tape
            .split(|x| *x == self.linebreak)
            .count() as u32;

        LinesState {
            linescount: self.linescount + line_breaks - 1,
            ..self
        }
    }
}

#[cfg(test)]
mod test {
    use crate::cw_lib::state::lines_state::LinesState;
    use crate::cw_lib::state::traits::{Compute, PartialState};
    use std::io::{BufReader, Read};
    use std::fs::File;

    #[test]
    pub fn test1(){
        let line = "hello world".as_bytes();
        let out = LinesState::new(b'\n').compute(line).output();
        assert_eq!(out,0)
    }
    #[test]
    pub fn test2(){
        let line = "".as_bytes();
        let out = LinesState::new(b'\n').compute(line).output();
        assert_eq!(out,0)
    }
    #[test]
    pub fn test3(){
        let line = "\n".as_bytes();
        let out = LinesState::new(b'\n').compute(line).output();
        assert_eq!(out,1)
    }
    #[test]
    pub fn test4(){
        let line = "hello\n".as_bytes();
        let out = LinesState::new(b'\n').compute(line).output();
        assert_eq!(out,1)
    }
    #[test]
    pub fn test5(){
        let line = "hello\nworld".as_bytes();
        let out = LinesState::new(b'\n').compute(line).output();
        assert_eq!(out,1)
    }
    #[test]
    pub fn test6(){
        let line = "\nworld".as_bytes();
        let out = LinesState::new(b'\n').compute(line).output();
        assert_eq!(out,1)
    }
    #[test]
    pub fn test7(){
        let line = "\nèô,sdfa".as_bytes();
        let out = LinesState::new(b'\n').compute(line).output();
        assert_eq!(out,1)
    }
    #[test]
    pub fn test8() {
        let out = LinesState::new(b'\n')
            .compute("helloworld".as_bytes())
            .compute("jksajksfjas a jkasjf da \n".as_bytes())
            .compute("\nsajisffajsjdfasf".as_bytes())
            .compute("hasisdaoasfo".as_bytes())
            .output();
        assert_eq!(out,2)
    }

    // Test on files
    fn proccess_file_test(f: &str) -> u32 {
        let mut reader = BufReader::new(File::open(f).unwrap());

        let mut state = LinesState::new(b'\n');
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
        let expected = 57;
        assert_eq!(out, expected)
    }

    #[test]
    fn lorem() {
        let out = proccess_file_test("tests/resources/Lorem_big.txt");
        assert_eq!(out, 1996)
    }
    #[test]
    fn bible() {
        let out = proccess_file_test("tests/resources/bible.txt");
        assert_eq!(out, 100182)
    }
    #[test]
    fn s1() {
        let out = proccess_file_test("tests/resources/sample1.txt");
        assert_eq!(out, 3)
    }

    #[test]
    fn s2() {
        let out = proccess_file_test("tests/resources/sample2.txt");
        assert_eq!(out, 12)
    }
    #[test]
    fn s3() {
        let out = proccess_file_test("tests/resources/sample3.txt");
        assert_eq!(out, 20)
    }
    #[test]
    fn small() {
        let out = proccess_file_test("tests/resources/small.txt");
        assert_eq!(out, 1)
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
        let expected = 0;
        assert_eq!(out, expected)
    }
    #[test]
    fn spanish() {
        let out = proccess_file_test("tests/resources/spanish.txt");
        let expected = 1;
        assert_eq!(out, expected)
    }

    #[test]
    fn french() {
        let out = proccess_file_test("tests/resources/french.txt");
        assert_eq!(out, 0)
    }
}