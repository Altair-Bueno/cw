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
    fn output(&self)->Self::Output{
        max(self.champion,self.buffer)
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
                if end { n_chars -= 1 }
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
        let out = MaxLengthState::new(b'\n').compute(line).output();
        assert_eq!(out,0)
    }
    #[test]
    pub fn test2() {
        let line = "hello\n".as_bytes();
        let out = MaxLengthState::new(b'\n').compute(line).output();
        assert_eq!(out,5)
    }
    #[test]
    pub fn test3() {
        let line = "hello\nworld".as_bytes();
        let out = MaxLengthState::new(b'\n').compute(line).output();
        assert_eq!(out,5)
    }
    #[test]
    pub fn test4() {
        let line = "hello\nworldjsafs\n".as_bytes();
        let out = MaxLengthState::new(b'\n').compute(line).output();
        assert_eq!(out,10)
    }
    #[test]
    pub fn test5() {
        let line = "hello\nworldjsafs\nshjksafhjkasfjhkfajshdjhksdfa".as_bytes();
        let out = MaxLengthState::new(b'\n').compute(line).output();
        assert_eq!(out,29)
    }
    #[test]
    pub fn test6() {
        let out = MaxLengthState::new(b'\n')
            .compute("hskjaskl a jadsjfjsdjk a asda dsfksa .".as_bytes())
            .compute("jkhsajkjafsdjkafsjkafsd".as_bytes())
            .compute("iassfdaafsd\n".as_bytes())
            .compute("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.".as_bytes())
            .output();
        assert_eq!(out,445)
    }
}