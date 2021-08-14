use crate::cw_lib::state::traits::{PartialState, Compute};
use std::cmp::max;

/// Max length
#[derive(Debug,Copy, Clone)]
pub struct MaxLengthState {
    buffer:u32,
    champion:u32,
    linebreak:u8,
}
impl Default for MaxLengthState {
    fn default() -> Self {
        MaxLengthState::new(b'\n')
    }
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
    type Output = u32;
    fn output(&self)->Result<Self::Output,String>{
        Ok(max(self.champion,self.buffer))
    }
}

impl Compute for MaxLengthState {
    fn compute(self, tape: &[u8]) -> Self {
        tape
            .split_inclusive(|x| self.linebreak == *x)
            .map(|x| {
                let mut n_chars = x.len();
                let end = if let Some(x) = x.last() {
                    *x == b'\n'
                } else {
                    false
                };
                if end { n_chars -= 1}
                // n_chars: number of chars without \n
                // end: If the line ended with \n or not
                (n_chars,end)
            })
            .fold(self,|acc,n| {
                let (this_len, buffer) = if n.1 {
                    (self.buffer + n.0 as u32, 0)
                } else {
                    (0,self.buffer + n.0 as u32)
                };
                MaxLengthState {
                    buffer,
                    champion: max(this_len,self.champion),
                    ..self
                }
            })
    }
}

#[cfg(test)]
mod test {
    use crate::cw_lib::state::max_length::MaxLengthState;
    use crate::cw_lib::state::traits::{Compute, PartialState};

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