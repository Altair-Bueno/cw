use crate::cw_lib::func::traits::{Compute, PartialState};
use regex::bytes::Regex;

// Number of words
#[derive(Debug,Copy, Clone)]
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
    fn output(&self) -> Result<u32, String> {
        Ok(self.wordcount)
    }
}

impl Compute for WordsState {
    fn compute(mut self, tape: &[u8]) -> Self {
        todo!();
        // Has at least one match
        let reg = Regex::new(r"(?P<frontspaces>[\x09\x20\x0A-\x0D]*)(?P<letters>[^\x09\x20\x0A-\x0D]*)").unwrap();

        let mut iter = reg.captures_iter(tape);

        let first = iter
            .next()
            .unwrap();

        let has_front_spaces = !first
            .name("frontspaces")
            .unwrap()
            .as_bytes()
            .is_empty();

        let has_letters = !first
            .name("letters")
            .unwrap()
            .as_bytes()
            .is_empty();

        let count = if self.onword && has_front_spaces {
            2
        } else if has_letters {
            1
        } else {
            0
        };

        let (count,onword) = iter.fold((count,has_letters), |(c,onw),n| {
            let temp = !n.name("letters").unwrap().as_bytes().is_empty();
            (c+1,temp)
        });

        WordsState {
            wordcount: self.wordcount + count,
            onword
        }
    }
}
#[cfg(test)]
mod test {
    use crate::cw_lib::func::words_state::WordsState;
    use crate::cw_lib::func::traits::{Compute, PartialState};

    #[test]
    pub fn test1() {
        let line = "".as_bytes();
        let out = WordsState::new().compute(line).output().unwrap();
        assert_eq!(out,0)
    }
    #[test]
    pub fn test2() {
        let line = "hello".as_bytes();
        let out = WordsState::new().compute(line).output().unwrap();
        assert_eq!(out,1)
    }
    #[test]
    pub fn test3() {
        let line = "hello world".as_bytes();
        let out = WordsState::new().compute(line).output().unwrap();
        assert_eq!(out,2)
    }
    #[test]
    pub fn test4() {
        let line = "hello\nworld".as_bytes();
        let out = WordsState::new().compute(line).output().unwrap();
        assert_eq!(out,2)
    }
    #[test]
    pub fn test5() {
        let line = "\nworld".as_bytes();
        let out = WordsState::new().compute(line).output().unwrap();
        assert_eq!(out,1)
    }
    #[test]
    pub fn test6() {
        let line = "\n\nworld".as_bytes();
        let out = WordsState::new().compute(line).output().unwrap();
        assert_eq!(out,1)
    }
    #[test]
    pub fn test7() {
        let line = "hello\n\n".as_bytes();
        let out = WordsState::new().compute(line).output().unwrap();
        assert_eq!(out,1)
    }
    #[test]
    pub fn test8() {
        let line = "texto en español de prueba con número de palabras".as_bytes();
        let out = WordsState::new().compute(line).output().unwrap();
        assert_eq!(out,9)
    }
    #[test]
    pub fn test9() {
        let line = "    \t   texto en      español de    prueba    con número\n\t \t de\n palabras".as_bytes();
        let out = WordsState::new().compute(line).output().unwrap();
        assert_eq!(out,9)
    }
}