use crate::cw_lib::func::traits::{PartialState, Compute};
use std::cmp::max;

/// Max length
#[derive(Debug,Copy, Clone)]
pub struct MaxLengthState {
    buffer:u32,
    champion:u32,
    linebreak:u8,
}
impl MaxLengthState {
    pub fn new(linebreak:u8) -> Self {
        MaxLengthState {
            buffer: 0,
            champion: 0,
            linebreak
        }
    }
}

impl PartialState for MaxLengthState {
    fn output(&self) -> Result<u32, String> {
        Ok(max(self.champion,self.buffer))
    }
}

impl Compute for MaxLengthState {
    fn compute(self, tape: &[u8]) -> Self {
        todo!();
        let (champion, buffer) = tape
            .split(|num| num == self.linebreak);
        /*
        .fold((self.champion,self.buffer), |(champion,buffer),slice| {

            ()
        });*/
    }
}

#[cfg(test)]
mod test {
    use crate::cw_lib::func::max_length::MaxLengthState;
    use crate::cw_lib::func::traits::{Compute, PartialState};

    #[test]
    pub fn test1() {
        let line = "".as_bytes();
        let out = MaxLengthState::new(b'\n').compute(line).output().unwrap();
        assert_eq!(out,0)
    }
    #[test]
    pub fn test2() {
        let line = "hello\n".as_bytes();
        let out = MaxLengthState::new(b'\n').compute(line).output().unwrap();
        assert_eq!(out,5)
    }
    #[test]
    pub fn test3() {
        let line = "hello\nworld".as_bytes();
        let out = MaxLengthState::new(b'\n').compute(line).output().unwrap();
        assert_eq!(out,5)
    }
    #[test]
    pub fn test4() {
        let line = "hello\nworldjsafs\n".as_bytes();
        let out = MaxLengthState::new(b'\n').compute(line).output().unwrap();
        assert_eq!(out,10)
    }
    #[test]
    pub fn test5() {
        let line = "hello\nworldjsafs\nshjksafhjkasfjhkfajshdjhksdfa".as_bytes();
        let out = MaxLengthState::new(b'\n').compute(line).output().unwrap();
        assert_eq!(out,29)
    }
}