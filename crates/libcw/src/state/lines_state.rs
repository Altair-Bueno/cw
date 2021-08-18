use crate::config::LineBreak;
use crate::state::traits::{compute::Compute, partial_state::PartialState};

/// number of lines
#[derive(Debug, Copy, Clone)]
pub struct LinesState {
    linescount: usize,
    linebreak: LineBreak,
}
impl Default for LinesState {
    fn default() -> Self {
        LinesState::new(LineBreak::LF)
    }
}

impl LinesState {
    pub fn new(linebreak: LineBreak) -> Self {
        LinesState {
            linescount: 0,
            linebreak,
        }
    }

    pub fn linebreak(&self) -> LineBreak {
        self.linebreak
    }
}

impl PartialState for LinesState {
    type Output = usize;
    fn output(&self) -> Self::Output {
        self.linescount
    }
}
impl Compute for LinesState {
    fn compute(self, tape: &[u8]) -> Self {
        let b = self.linebreak.get_separator();
        let line_breaks = tape.iter().filter(|x| **x == b).count();
        LinesState {
            linescount: line_breaks + self.linescount,
            linebreak: self.linebreak,
        }
    }
}

#[cfg(test)]
mod test {
    use std::fs::File;
    use std::io::{BufReader, Read};

    use crate::config::LineBreak;
    use crate::state::lines_state::LinesState;
    use crate::state::traits::{compute::Compute, partial_state::PartialState};

    #[test]
    pub fn test1() {
        let line = "hello world".as_bytes();
        let out = LinesState::new(LineBreak::LF).compute(line).output();
        assert_eq!(out, 0)
    }

    #[test]
    pub fn test2() {
        let line = "".as_bytes();
        let out = LinesState::new(LineBreak::LF).compute(line).output();
        assert_eq!(out, 0)
    }
    #[test]
    pub fn test3() {
        let line = "\n".as_bytes();
        let out = LinesState::new(LineBreak::LF).compute(line).output();
        assert_eq!(out, 1)
    }
    #[test]
    pub fn test4() {
        let line = "hello\n".as_bytes();
        let out = LinesState::new(LineBreak::LF).compute(line).output();
        assert_eq!(out, 1)
    }
    #[test]
    pub fn test5() {
        let line = "hello\nworld".as_bytes();
        let out = LinesState::new(LineBreak::LF).compute(line).output();
        assert_eq!(out, 1)
    }
    #[test]
    pub fn test6() {
        let line = "\nworld".as_bytes();
        let out = LinesState::new(LineBreak::LF).compute(line).output();
        assert_eq!(out, 1)
    }
    #[test]
    pub fn test7() {
        let line = "\nèô,sdfa".as_bytes();
        let out = LinesState::new(LineBreak::LF).compute(line).output();
        assert_eq!(out, 1)
    }
    #[test]
    pub fn test8() {
        let out = LinesState::new(LineBreak::LF)
            .compute("helloworld".as_bytes())
            .compute("jksajksfjas a jkasjf da \n".as_bytes())
            .compute("\nsajisffajsjdfasf".as_bytes())
            .compute("hasisdaoasfo".as_bytes())
            .output();
        assert_eq!(out, 2)
    }

    // Test on files
    fn proccess_file_test(f: &str) -> usize {
        let mut reader = BufReader::new(File::open(f).unwrap());

        let mut state = LinesState::new(LineBreak::LF);
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
        let out = proccess_file_test("resources/Gabriel.txt");
        let expected = 57;
        assert_eq!(out, expected)
    }

    #[test]
    fn lorem() {
        let out = proccess_file_test("resources/Lorem_big.txt");
        assert_eq!(out, 1996)
    }
    #[test]
    #[ignore]
    fn world192() {
        let out = proccess_file_test("resources/world192.txt");
        assert_eq!(out, 65119)
    }
    #[test]
    fn s1() {
        let out = proccess_file_test("resources/sample1.txt");
        assert_eq!(out, 3)
    }

    #[test]
    fn s2() {
        let out = proccess_file_test("resources/sample2.txt");
        assert_eq!(out, 12)
    }
    #[test]
    fn s3() {
        let out = proccess_file_test("resources/sample3.txt");
        assert_eq!(out, 20)
    }
    #[test]
    fn small() {
        let out = proccess_file_test("resources/small.txt");
        assert_eq!(out, 1)
    }
    #[test]
    fn empty() {
        let out = proccess_file_test("resources/empty.txt");
        assert_eq!(out, 0)
    }

    #[test]
    fn arabic() {
        // - Legth isn't 0
        // - test weird
        let out = proccess_file_test("resources/arabic.txt");
        let expected = 0;
        assert_eq!(out, expected)
    }
    #[test]
    fn spanish() {
        let out = proccess_file_test("resources/spanish.txt");
        let expected = 1;
        assert_eq!(out, expected)
    }

    #[test]
    fn french() {
        let out = proccess_file_test("resources/french.txt");
        assert_eq!(out, 0)
    }
}
