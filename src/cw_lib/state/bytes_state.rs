use crate::cw_lib::state::traits::{PartialState, Compute};

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
    type Output = u32;
    fn output(&self)->Self::Output{
        self.bytecount
    }
}

impl Compute for BytesState {
    fn compute(mut self, tape: &[u8]) -> Self {
        self.bytecount += (tape.len() as u32);
        self
    }
}

#[cfg(test)]
mod test {
    use crate::cw_lib::state::bytes_state::BytesState;
    use crate::cw_lib::state::traits::{Compute, PartialState};

    #[test]
    pub fn test1 () {
        let bytes = "hello world".as_bytes();
        let parse = BytesState::new().compute(bytes).output();
        assert_eq!(parse,11)
    }
    #[test]
    pub fn test2 () {
        let bytes = "".as_bytes();
        let parse = BytesState::new().compute(bytes).output();
        assert_eq!(parse,0)
    }
    #[test]
    pub fn test3 () {
        let bytes = "ñ".as_bytes();
        let parse = BytesState::new().compute(bytes).output();
        assert_eq!(parse,2)
    }
    #[test]
    pub fn test4() {
        let parse = BytesState::new()
            .compute("ñ".as_bytes())
            .compute("hello".as_bytes())
            .compute(" ass sa fda fsj fasd ".as_bytes())
            .output();
        assert_eq!(parse,28)
    }

}
