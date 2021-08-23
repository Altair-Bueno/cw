use crate::state::traits::{compute::Compute, partial_state::PartialState};

/// Represents a partial state that can be used as a middle structure to
/// store partial reads over a divided tape
#[derive(Default, Debug, Copy, Clone)]
pub struct BytesState {
    bytecount: usize,
}
impl BytesState {
    /// Creates a new instance of BytesState
    pub fn new() -> Self {
        Default::default()
    }
    fn count(self , tape:&[u8]) -> Self {
        BytesState {
            bytecount: self.bytecount + tape.len()
        }
    }
}

impl PartialState for BytesState {
    type Output = usize;
    fn output(&self) -> Self::Output {
        self.bytecount
    }
}

impl Compute for BytesState {
    fn utf8_compute(self, tape: &[u8]) -> Self {
        self.count(tape)
    }

    fn utf16_compute(self, tape: &[u8]) -> Self {
        self.count(tape)
    }
}

#[cfg(test)]
    mod test {
    mod utf16 {
        use std::fs::File;
        use std::io::{BufReader, Read};

        use crate::state::bytes_state::BytesState;
        use crate::state::traits::{compute::Compute, partial_state::PartialState};

        #[test]
        pub fn test1() {
            let bytes = "hello world".encode_utf16().flat_map(u16::to_be_bytes).collect::<Vec<u8>>();
            let parse = BytesState::new().utf16_compute(bytes.as_slice()).output();
            assert_eq!(parse, 22)
        }

        #[test]
        pub fn test2() {
            let bytes = "".encode_utf16().flat_map(u16::to_be_bytes).collect::<Vec<u8>>();
            let parse = BytesState::new().utf16_compute(bytes.as_slice()).output();
            assert_eq!(parse, 0)
        }

        #[test]
        pub fn test3() {
            let bytes = "ñ".encode_utf16().flat_map(u16::to_be_bytes).collect::<Vec<u8>>();
            let parse = BytesState::new().utf16_compute(bytes.as_slice()).output();
            assert_eq!(parse, 2)
        }

        #[test]
        pub fn test4() {
            let s1 = "ñ".encode_utf16().flat_map(u16::to_be_bytes).collect::<Vec<u8>>();
            let s2 = "hello".encode_utf16().flat_map(u16::to_be_bytes).collect::<Vec<u8>>();
            let s3 = " ass sa fda fsj fasd ".encode_utf16().flat_map(u16::to_be_bytes).collect::<Vec<u8>>();
            let parse = BytesState::new()
                .utf16_compute(s1.as_slice())
                .utf16_compute(s2.as_slice())
                .utf16_compute(s3.as_slice())
                .output();
            assert_eq!(parse, 54)
        }
    }

    mod utf8 {
    use std::fs::File;
    use std::io::{BufReader, Read};

    use crate::state::bytes_state::BytesState;
    use crate::state::traits::{compute::Compute, partial_state::PartialState};

    # [test]
    pub fn test1() {
    let bytes = "hello world".as_bytes();
    let parse = BytesState::new().utf8_compute(bytes).output();
    assert_eq ! (parse, 11)
    }

    # [test]
    pub fn test2() {
    let bytes = "".as_bytes();
    let parse = BytesState::new().utf8_compute(bytes).output();
    assert_eq ! (parse, 0)
    }
    # [test]
    pub fn test3() {
    let bytes = "ñ".as_bytes();
    let parse = BytesState::new().utf8_compute(bytes).output();
    assert_eq ! (parse, 2)
    }
    # [test]
    pub fn test4() {
    let parse = BytesState::new()
    .utf8_compute("ñ".as_bytes())
    .utf8_compute("hello".as_bytes())
    .utf8_compute(" ass sa fda fsj fasd ".as_bytes())
    .output();
    assert_eq ! (parse, 28)
    }

    // Test on files
    fn proccess_file_test(f: & str) -> usize {
    let mut reader = BufReader::new(File::open(f).unwrap());

    let mut state = BytesState::new();
    let mut buff = [0; 1024];
    loop {
    let read = reader.read( & mut buff).unwrap();
    if read == 0 {
    return state.output();
    }
    state = state.utf8_compute( & buff[0..read]);
    }
    }

    # [test]
    fn gabriel() {
    let out = proccess_file_test("resources/utf8/Gabriel.txt");
    let expected = 2700;
    assert_eq ! (out, expected)
    }

    # [test]
    fn lorem() {
    let out = proccess_file_test("resources/utf8/Lorem_big.txt");
    assert_eq !(out, 751539)
    }
    # [test]
    fn s1() {
    let out = proccess_file_test("resources/utf8/sample1.txt");
    assert_eq !(out, 607)
    }

    # [test]
    fn s2() {
    let out = proccess_file_test("resources/utf8/sample2.txt");
    assert_eq !(out, 2859)
    }
    # [test]
    fn s3() {
    let out = proccess_file_test("resources/utf8/sample3.txt");
    assert_eq !(out, 3541)
    }
    # [test]
    fn small() {
    let out = proccess_file_test("resources/utf8/small.txt");
    assert_eq !(out, 18)
    }
    # [test]
    fn empty() {
    let out = proccess_file_test("resources/utf8/empty.txt");
    assert_eq !(out, 0)
    }

    # [test]
    fn arabic() {
    // - Legth isn't 0
    // - test weird
    let out = proccess_file_test("resources/utf8/arabic.txt");
    let expected = 105;
    assert_eq ! (out, expected)
    }
    # [test]
    fn spanish() {
    let out = proccess_file_test("resources/utf8/spanish.txt");
    let expected = 22;
    assert_eq ! (out, expected)
    }

    # [test]
    fn french() {
    let out = proccess_file_test("resources/utf8/french.txt");
    assert_eq !(out, 61)
    }
    }
}