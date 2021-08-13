use crate::cw_lib::func::traits::{PartialState, Compute};

/// Number of bytes
#[derive(Default,Debug,Copy, Clone)]
pub struct BytesState {
    bytecount:u32,
}
impl BytesState {
    pub fn new() -> Self {
        Default::default()
    }
}

impl PartialState for BytesState {
    fn output(&self) -> Result<u32,String> {
        Ok(self.bytecount)
    }
}

impl Compute for BytesState {
    fn compute(mut self, tape: &[u8]) -> Self {
        self.bytecount += tape.len();
        self
    }
}

#[cfg(test)]
mod test {
    use crate::cw_lib::func::bytes_state::BytesState;
    use crate::cw_lib::func::traits::{Compute, PartialState};

    #[test]
    pub fn test1 () {
        let bytes = "hello world".as_bytes();
        let parse = BytesState::new().compute(bytes).output().unwrap();
        assert_eq!(parse,11)
    }
    #[test]
    pub fn test2 () {
        let bytes = "".as_bytes();
        let parse = BytesState::new().compute(bytes).output().unwrap();
        assert_eq!(parse,0)
    }
    #[test]
    pub fn test3 () {
        let bytes = "ñ".as_bytes();
        let parse = BytesState::new().compute(bytes).output().unwrap();
        assert_eq!(parse,2)
    }

}
