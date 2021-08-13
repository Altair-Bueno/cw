use crate::cw_lib::func::traits::{PartialState, Compute};

/// number of lines
#[derive(Debug,Copy, Clone)]
pub struct LinesState {
    linescount:u32,
    linebreak:u8,
}
impl LinesState {
    pub fn new(linebreak:u8) -> Self {
        LinesState{ linescount: 0, linebreak }
    }
}

impl PartialState for LinesState {
    fn output(&self) -> Result<u32, String> {
        Ok(self.linescount)
    }
}
impl Compute for LinesState {
    fn compute(self, tape: &[u8]) -> Self {
        let line_breaks = tape
            .split(|x| x == self.linebreak)
            .count();

        LinesState {
            linescount: self.linescount + line_breaks,
            ..self
        }
    }
}

#[cfg(test)]
mod test {
    use crate::cw_lib::func::lines_state::LinesState;
    use crate::cw_lib::func::traits::{Compute, PartialState};

    #[test]
    pub fn test1(){
        let line = "hello world".as_bytes();
        let out = LinesState::new(b'\n').compute(line).output().unwrap();
        assert_eq!(out,0)
    }
    #[test]
    pub fn test2(){
        let line = "".as_bytes();
        let out = LinesState::new(b'\n').compute(line).output().unwrap();
        assert_eq!(out,0)
    }
    #[test]
    pub fn test3(){
        let line = "\n".as_bytes();
        let out = LinesState::new(b'\n').compute(line).output().unwrap();
        assert_eq!(out,1)
    }
    #[test]
    pub fn test4(){
        let line = "hello\n".as_bytes();
        let out = LinesState::new(b'\n').compute(line).output().unwrap();
        assert_eq!(out,1)
    }
    #[test]
    pub fn test5(){
        let line = "hello\nworld".as_bytes();
        let out = LinesState::new(b'\n').compute(line).output().unwrap();
        assert_eq!(out,1)
    }
    #[test]
    pub fn test6(){
        let line = "\nworld".as_bytes();
        let out = LinesState::new(b'\n').compute(line).output().unwrap();
        assert_eq!(out,1)
    }
    #[test]
    pub fn test7(){
        let line = "\nèô,sdfa".as_bytes();
        let out = LinesState::new(b'\n').compute(line).output().unwrap();
        assert_eq!(out,1)
    }
}