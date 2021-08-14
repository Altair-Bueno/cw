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
        (self.linescount)
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
}