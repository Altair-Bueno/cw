use crate::cw_lib::state::traits::{Compute, PartialState};
use regex::bytes::Regex;

// Number of words
#[derive(Default,Debug,Copy, Clone)]
pub struct WordsState{
    wordcount:u32,
    onword:bool,
}

impl WordsState {
    pub fn new() ->Self {
        Default::default()
    }
}

impl PartialState for WordsState {
    type Output = u32;
    fn output(&self)->Self::Output{
        let remaining = if self.onword {
            1
        } else {
            0
        };
        self.wordcount + remaining
    }
}

impl Compute for WordsState {
    fn compute(mut self, tape: &[u8]) -> Self {
        let reg = Regex::new(r"([\x09\x20\x0A-\x0D]*)[^\x09\x20\x0A-\x0D]+([\x09\x20\x0A-\x0D]*)").unwrap();

        reg.captures_iter(tape)
            .map(|x|
                (
                    x.get(1).unwrap().as_bytes().len(),
                    x.get(2).unwrap().as_bytes().len()
                )
            )
            .fold(self,|acc,spaces| {
                let (this,onword) = match spaces {
                    // Palabra cortada, sigue siendo la misma palabra
                    // (0,0) if acc.onword => (0,true), // simplified
                    (0,0)               => (0,true),
                    (_,0) if acc.onword => (1,true),
                    (_,0)               => (0,true),
                    (0,_) if acc.onword => (1,false),
                    (0,_)               => (1,false),
                    _ if acc.onword     => (2,false),
                    _                   => (1,false),
                };

                WordsState {
                    wordcount: this + acc.wordcount,
                    onword
                }
            })
    }
}
#[cfg(test)]
mod test {
    use crate::cw_lib::state::words_state::WordsState;
    use crate::cw_lib::state::traits::{Compute, PartialState};

    #[test]
    pub fn test1() {
        let line = "".as_bytes();
        let out = WordsState::new().compute(line).output();
        assert_eq!(out,0)
    }
    #[test]
    pub fn test2() {
        let line = "hello".as_bytes();
        let out = WordsState::new().compute(line).output();
        assert_eq!(out,1)
    }
    #[test]
    pub fn test3() {
        let line = "hello world".as_bytes();
        let out = WordsState::new().compute(line).output();
        assert_eq!(out,2)
    }
    #[test]
    pub fn test4() {
        let line = "hello\nworld".as_bytes();
        let out = WordsState::new().compute(line).output();
        assert_eq!(out,2)
    }
    #[test]
    pub fn test5() {
        let line = "\nworld".as_bytes();
        let out = WordsState::new().compute(line).output();
        assert_eq!(out,1)
    }
    #[test]
    pub fn test6() {
        let line = "\n\nworld".as_bytes();
        let out = WordsState::new().compute(line).output();
        assert_eq!(out,1)
    }
    #[test]
    pub fn test7() {
        let line = "hello\n\n".as_bytes();
        let out = WordsState::new().compute(line).output();
        assert_eq!(out,1)
    }
    #[test]
    pub fn test8() {
        let line = "texto en español de prueba con número de palabras".as_bytes();
        let out = WordsState::new().compute(line).output();
        assert_eq!(out,9)
    }
    #[test]
    pub fn test9() {
        let line = "    \t   texto en      español de    prueba    con número\n\t \t de\n palabras".as_bytes();
        let out = WordsState::new().compute(line).output();
        assert_eq!(out,9)
    }
    #[test]
    pub fn test10() {
        let out = WordsState::new()
            .compute("hell".as_bytes())
            .compute("o ".as_bytes())
            .compute("world".as_bytes())
            .output();
        assert_eq!(out,2)
    }
}