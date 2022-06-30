use crate::state::traits::{compute::Compute, partial_state::PartialState};

#[derive(Copy, Clone, Debug, Default)]
pub struct CharState {
    expect: usize,
    num_chars: usize,
}

impl CharState {
    pub fn new() -> CharState {
        Default::default()
    }

    fn compute(self, tape: &[u8], decoder: fn(u8) -> usize) -> CharState {
        tape.iter().fold(self, |acc, n| {
            let (expect, num_chars) = if acc.expect != 0 {
                (acc.expect - 1, acc.num_chars)
            } else {
                ((decoder)(*n), acc.num_chars + 1)
            };
            CharState { expect, num_chars }
        })
    }

    fn utf16_decoder(n: u8) -> usize {
        let shift = n >> 2;
        let mask: u8 = 0b00110110;
        if shift == mask {
            3
        } else {
            1
        }
    }

    fn utf8_decoder(n: u8) -> usize {
        let three: u8 = 0b11110000;
        let two: u8 = 0b11100000;
        let one: u8 = 0b11000000;
        if n & three == three {
            // 11110uuu 10uuzzzz 10yyyyyy 10xxxxxx
            3
        } else if n & two == two {
            // 1110zzzz 10yyyyyy 10xxxxxx
            2
        } else if n & one == one {
            // 110yyyyy 10xxxxxx
            1
        } else {
            0
        }
    }
}

impl PartialState for CharState {
    type Output = usize;

    fn output(&self) -> Self::Output {
        self.num_chars
    }
}

impl Compute for CharState {
    fn utf8_compute(self, tape: &[u8]) -> Self {
        self.compute(tape, CharState::utf8_decoder)
    }

    fn utf16_compute(self, tape: &[u8]) -> Self {
        self.compute(tape, CharState::utf16_decoder)
    }
}

#[cfg(test)]
mod test {
    use rstest::*;
    use speculoos::assert_that;

    use crate::state::chars_state::CharState;
    use crate::state::traits::compute::Compute;
    use crate::state::traits::partial_state::PartialState;

    #[fixture]
    fn char_state() -> CharState {
        CharState::new()
    }

    #[rstest]
    #[case("", 0)]
    #[case(" ", 1)]
    #[case("ñ", 1)]
    #[case("/", 1)]
    #[case("hello", 5)]
    #[case("ñó", 2)]
    #[trace]
    fn utf8_contains_the_expected_amount_of_characters(char_state: CharState, #[case] string: &str, #[case] expected: usize) {
        let utf8_encoded = string.as_bytes();

        let obtained = char_state.utf8_compute(utf8_encoded).output();

        assert_that!(obtained).is_equal_to(expected)
    }

    #[rstest]
    #[case("", 0)]
    #[case(" ", 1)]
    #[case("ñ", 1)]
    #[case("/", 1)]
    #[case("hello", 5)]
    #[case("ñó", 2)]
    #[trace]
    fn utf16be_contains_the_expected_amount_of_characters(char_state: CharState, #[case] string: &str, #[case] expected: usize) {
        let utf16_encoded: Vec<_> = string.encode_utf16().flat_map(|x| x.to_be_bytes()).collect();

        let obtained = char_state.utf16_compute(utf16_encoded.as_slice()).output();

        assert_that!(obtained).is_equal_to(expected)
    }
}
