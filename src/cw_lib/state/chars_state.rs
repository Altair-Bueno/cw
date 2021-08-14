use crate::cw_lib::state::traits::{PartialState, Compute};
use regex::bytes::Regex;
use regex::bytes::RegexSet;

#[derive(Copy, Clone)]
pub enum Expect {
    New,
    One,
    Two,
    Three,
    Four
}
impl Default for Expect {
    fn default() -> Self {
        Expect::New
    }
}

// TODO
#[derive(Default,Copy, Clone)]
pub struct CharState{
    expect: Expect,
    num_chars:u32
}

impl CharState {
    pub fn new()-> CharState{
        Default::default()
    }
}
impl PartialState for CharState {
    type Output = u32;

    fn output(&self)->Self::Output {
        self.num_chars
    }
}
impl Compute for CharState {
    fn compute(mut self, tape: &[u8]) -> Self {
        if tape.is_empty() {
            return self
        }
        // todo comerle por delante a la cinta
/*
        let four = Regex::new(r"\b11110000...").unwrap();
        let triple = Regex::new(r"\b11100000..").unwrap();
        let double = Regex::new(r"\b11000000.").unwrap();
        let single = Regex::new(r".").unwrap();
        */
        let reg = Regex::new(r"(\xF0...|\xE0..|\xC0.|.)").unwrap();
        let num = reg.captures_iter(tape).count();

        CharState {
            expect: Default::default(),
            num_chars: self.num_chars + num as u32
        }
    }
}

#[cfg(test)]
mod test {
    use crate::cw_lib::state::chars_state::CharState;
    use crate::cw_lib::state::traits::{Compute, PartialState};

    #[test]
    pub fn test1() {
        let s = "hello world".as_bytes();
        let out = CharState::new().compute(s).output();
        assert_eq!(out,11)
    }
}