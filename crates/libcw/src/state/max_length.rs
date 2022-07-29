use std::cmp::max;

use crate::config::LineBreak;
use crate::state::chars::CharState;
use crate::traits::{compute::Compute, partial_state::PartialState};

/// Max length
#[derive(Default, Debug, Copy, Clone)]
pub struct MaxLengthState {
    max_length_found: usize,
    //line_count: usize,
    //char_count:usize,
    linebreak: LineBreak,
    char_state: CharState,
}

impl MaxLengthState {
    pub fn new(linebreak: LineBreak) -> Self {
        MaxLengthState {
            max_length_found: 0,
            //line_count: 0,
            //char_count: 0,
            linebreak,
            char_state: CharState::new(),
        }
    }
}

impl PartialState for MaxLengthState {
    type Output = usize;
    fn output(self) -> Self::Output {
        let char_state_output = self.char_state.output();
        max(self.max_length_found, char_state_output)
        // let line_count = self.line_count;
        // let character_count = char_state_output + self.char_count;
    }
}

impl Compute for MaxLengthState {
    fn utf8_compute(self, tape: &[u8]) -> Self {
        let b = self.linebreak.get_separator();
        tape.split_inclusive(|x| *x == b).fold(self, |state, next| {
            let on_line = next.last().map(|x| *x != b).unwrap_or(true);
            let count_chars_state = state.char_state.utf8_compute(next);
            // Count lines if it's the end of the line. Update character
            // count in the end
            if on_line {
                // No linebreak detected. Still same line as before
                MaxLengthState {
                    char_state: count_chars_state,
                    ..state
                }
            } else {
                let count = count_chars_state.output();
                MaxLengthState {
                    max_length_found: max(count - 1, state.max_length_found),
                    //line_count: state.line_count + 1,
                    //char_count: state.char_count + count,
                    char_state: CharState::new(),
                    ..state
                }
            }
        })
    }

    fn utf16_compute(self, tape: &[u8]) -> Self {
        let b = self.linebreak.get_separator();
        tape.split_inclusive(|x| *x == b).fold(self, |state, next| {
            let on_line = next.last().map(|x| *x != b).unwrap_or(true);
            let count_chars_state = state.char_state.utf16_compute(next);
            // Count lines if it's the end of the line. Update character
            // count in the end
            if on_line {
                // No linebreak detected. Still same line as before
                MaxLengthState {
                    char_state: count_chars_state,
                    ..state
                }
            } else {
                let count = count_chars_state.output();
                MaxLengthState {
                    max_length_found: max(count - 1, state.max_length_found),
                    //line_count: state.line_count + 1,
                    //char_count: state.char_count + count,
                    char_state: CharState::new(),
                    ..state
                }
            }
        })
    }
}

#[cfg(test)]
mod test {
    use rstest::*;
    use speculoos::assert_that;

    use crate::config::LineBreak;
    use crate::state::max_length::MaxLengthState;
    use crate::traits::compute::Compute;
    use crate::traits::partial_state::PartialState;

    #[fixture]
    fn max_length_state(#[default(LineBreak::LF)] linebreak: LineBreak) -> MaxLengthState {
        MaxLengthState::new(linebreak)
    }

    #[rstest]
    #[case("", 0)]
    #[case("Hello world", 11)]
    #[case("H\nello", 4)]
    #[case("Hello\n wor", 5)]
    #[case("One\ntwo\t\nanother", 7)]
    #[trace]
    fn utf8_lf_has_the_expected_max_length(
        max_length_state: MaxLengthState,
        #[case] string: &str,
        #[case] expected: usize,
    ) {
        let utf8_encoded = string.as_bytes();

        let obtained = max_length_state.utf8_compute(utf8_encoded).output();

        assert_that!(obtained).is_equal_to(expected)
    }

    #[rstest]
    #[case("", 0)]
    #[case("Hello world", 11)]
    #[case("H\nello", 4)]
    #[case("Hello\n wor", 5)]
    #[case("One\ntwo\t\nanother", 7)]
    #[trace]
    fn utf16be_lf_has_the_expected_max_length(
        max_length_state: MaxLengthState,
        #[case] string: &str,
        #[case] expected: usize,
    ) {
        let utf16_encoded: Vec<_> = string
            .encode_utf16()
            .flat_map(|x| x.to_be_bytes())
            .collect();

        let obtained = max_length_state
            .utf16_compute(utf16_encoded.as_slice())
            .output();

        assert_that!(obtained).is_equal_to(expected)
    }

    #[rstest]
    #[case("", 0)]
    #[case("Hello world", 11)]
    #[case("H\rello", 4)]
    #[case("Hello\r wor", 5)]
    #[case("One\rtwo\t\ranother", 7)]
    #[trace]
    fn utf8_cr_has_the_expected_max_length(
        #[with(LineBreak::CR)] max_length_state: MaxLengthState,
        #[case] string: &str,
        #[case] expected: usize,
    ) {
        let utf8_encoded = string.as_bytes();

        let obtained = max_length_state.utf8_compute(utf8_encoded).output();

        assert_that!(obtained).is_equal_to(expected)
    }

    #[rstest]
    #[case("", 0)]
    #[case("Hello world", 11)]
    #[case("H\rello", 4)]
    #[case("Hello\r wor", 5)]
    #[case("One\rtwo\t\ranother", 7)]
    #[trace]
    fn utf16be_cr_has_the_expected_max_length(
        #[with(LineBreak::CR)] max_length_state: MaxLengthState,
        #[case] string: &str,
        #[case] expected: usize,
    ) {
        let utf16_encoded: Vec<_> = string
            .encode_utf16()
            .flat_map(|x| x.to_be_bytes())
            .collect();

        let obtained = max_length_state
            .utf16_compute(utf16_encoded.as_slice())
            .output();

        assert_that!(obtained).is_equal_to(expected)
    }
}
