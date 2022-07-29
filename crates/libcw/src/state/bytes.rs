use crate::traits::{compute::Compute, partial_state::PartialState};

/// Represents a partial state that can be used as a middle structure to
/// store partial reads over a divided tape
#[derive(Default, Debug, Copy, Clone)]
pub struct BytesState {
    byte_count: usize,
}

impl BytesState {
    /// Creates a new instance of BytesState
    pub fn new() -> Self {
        Default::default()
    }
    fn count(self, tape: &[u8]) -> Self {
        BytesState {
            byte_count: self.byte_count + tape.len(),
        }
    }
}

impl PartialState for BytesState {
    type Output = usize;
    fn output(self) -> Self::Output {
        self.byte_count
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
    use rstest::*;
    use speculoos::assert_that;

    use crate::state::bytes::BytesState;
    use crate::traits::compute::Compute;
    use crate::traits::partial_state::PartialState;

    #[fixture]
    pub fn bytes_state() -> BytesState {
        BytesState::new()
    }

    #[rstest]
    #[case("", 0)]
    #[case("Hello world", 11)]
    #[case("ñ", 2)]
    #[case("\r", 1)]
    #[case("❤", 3)]
    #[trace]
    fn utf8_has_the_expected_bytesize(
        bytes_state: BytesState,
        #[case] string: &str,
        #[case] expected: usize,
    ) {
        let utf8_encoded = string.as_bytes();

        let obtained = bytes_state.utf8_compute(utf8_encoded).output();

        assert_that!(obtained).is_equal_to(expected)
    }

    #[rstest]
    #[case("", 0)]
    #[case("Hello world", 22)]
    #[case("ñ", 2)]
    #[case("\r", 2)]
    #[case("❤", 2)]
    #[trace]
    fn utf16be_has_the_expected_bytesize(
        bytes_state: BytesState,
        #[case] string: &str,
        #[case] expected: usize,
    ) {
        let utf8_encoded: Vec<_> = string
            .encode_utf16()
            .flat_map(|x| x.to_be_bytes())
            .collect();

        let obtained = bytes_state.utf8_compute(utf8_encoded.as_slice()).output();

        assert_that!(obtained).is_equal_to(expected)
    }
}
