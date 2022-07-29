use crate::state::traits::{compute::Compute, partial_state::PartialState};

// Number of words
#[derive(Copy, Clone, Default, Debug)]
pub struct WordsState {
    wordcount: usize,
    onword: bool,
}

impl WordsState {
    pub fn new() -> Self {
        Default::default()
    }

    fn compute_char(self, n: &u8) -> WordsState {
        let is_separator = |x: u8| -> bool {
            match x {
                0x20 | 0x09 => true,
                x => (0x0A..=0x0D).contains(&x),
            }
        };
        let onword = !is_separator(*n);
        let wordcount = self.wordcount + {
            if self.onword && !onword {
                1
            } else {
                0
            }
        };

        WordsState { wordcount, onword }
    }
}

impl PartialState for WordsState {
    type Output = usize;
    fn output(&self) -> Self::Output {
        let remaining = if self.onword { 1 } else { 0 };
        self.wordcount + remaining
    }
}

impl Compute for WordsState {
    fn utf8_compute(self, tape: &[u8]) -> Self {
        tape.iter().fold(self, WordsState::compute_char)
    }

    fn utf16_compute(self, tape: &[u8]) -> Self {
        let mut temp = true;
        tape.iter()
            .filter(|_| {
                temp = !temp;
                temp
            })
            .fold(self, WordsState::compute_char)
    }
}

#[cfg(test)]
mod test {
    use rstest::*;
    use speculoos::assert_that;

    use crate::state::traits::compute::Compute;
    use crate::state::traits::partial_state::PartialState;
    use crate::state::words_state::WordsState;

    #[fixture]
    fn words_state() -> WordsState {
        WordsState::new()
    }

    #[rstest]
    #[case("", 0)]
    #[case("Hello", 1)]
    #[case("Hello ", 1)]
    #[case(" Hello", 1)]
    #[case(" Hello ", 1)]
    #[case("Hello world ", 2)]
    #[case("Hello\nworld ", 2)]
    #[case("Hello\rworld ", 2)]
    #[trace]
    fn utf8_contains_the_expected_amount_of_words(
        words_state: WordsState,
        #[case] string: &str,
        #[case] expected: usize,
    ) {
        let utf8_encoded = string.as_bytes();

        let obtained = words_state.utf8_compute(utf8_encoded).output();

        assert_that!(obtained).is_equal_to(expected)
    }

    #[rstest]
    #[case("", 0)]
    #[case("Hello", 1)]
    #[case("Hello ", 1)]
    #[case(" Hello", 1)]
    #[case(" Hello ", 1)]
    #[case("Hello world ", 2)]
    #[case("Hello\nworld ", 2)]
    #[case("Hello\rworld ", 2)]
    #[trace]
    fn utf16be_contains_the_expected_amount_of_words(
        words_state: WordsState,
        #[case] string: &str,
        #[case] expected: usize,
    ) {
        let utf16_encoded: Vec<_> = string
            .encode_utf16()
            .flat_map(|x| x.to_be_bytes())
            .collect();

        let obtained = words_state.utf16_compute(utf16_encoded.as_slice()).output();

        assert_that!(obtained).is_equal_to(expected)
    }
}
