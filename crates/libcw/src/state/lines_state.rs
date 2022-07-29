use std::fmt::Debug;

use crate::config::LineBreak;
use crate::state::traits::{compute::Compute, partial_state::PartialState};

/// number of lines
#[derive(Default, Debug, Copy, Clone)]
pub struct LinesState {
    line_count: usize,
    linebreak: LineBreak,
}

impl LinesState {
    pub fn new(linebreak: LineBreak) -> Self {
        LinesState {
            line_count: 0,
            linebreak,
        }
    }
}

impl PartialState for LinesState {
    type Output = usize;
    fn output(self) -> Self::Output {
        self.line_count
    }
}

impl Compute for LinesState {
    fn utf8_compute(self, tape: &[u8]) -> Self {
        let b = self.linebreak.get_separator();
        let line_breaks = tape.iter().filter(|x| **x == b).count();

        LinesState {
            line_count: line_breaks + self.line_count,
            ..self
        }
    }
    fn utf16_compute(self, tape: &[u8]) -> Self {
        let b = self.linebreak.get_separator();
        let mut temp = true;
        let line_breaks = tape
            .iter()
            .filter(|_| {
                temp = !temp;
                temp
            })
            .filter(|x| **x == b)
            .count();
        LinesState {
            line_count: line_breaks + self.line_count,
            ..self
        }
    }
}

#[cfg(test)]
mod test {
    use rstest::*;
    use speculoos::assert_that;

    use crate::config::LineBreak;
    use crate::state::lines_state::LinesState;
    use crate::state::traits::compute::Compute;
    use crate::state::traits::partial_state::PartialState;

    #[fixture]
    fn lines_state(#[default(LineBreak::LF)] linebreak: LineBreak) -> LinesState {
        LinesState::new(linebreak)
    }

    #[rstest]
    #[case("", 0)]
    #[case("Hello world", 0)]
    #[case("This is some \n long text", 1)]
    #[case("\n", 1)]
    #[case("\n\n", 2)]
    #[case(" \n", 1)]
    #[trace]
    fn utf8_lf_contains_the_expected_amount_of_line_breaks(
        lines_state: LinesState,
        #[case] string: &str,
        #[case] expected: usize,
    ) {
        let utf8_encoded = string.as_bytes();

        let obtained = lines_state.utf8_compute(utf8_encoded).output();

        assert_that!(obtained).is_equal_to(expected)
    }

    #[rstest]
    #[case("", 0)]
    #[case("Hello world", 0)]
    #[case("This is some \n long text", 1)]
    #[case("\n", 1)]
    #[case("\n\n", 2)]
    #[case(" \n", 1)]
    #[trace]
    fn utf16be_lf_contains_the_expected_amount_of_line_breaks(
        lines_state: LinesState,
        #[case] string: &str,
        #[case] expected: usize,
    ) {
        let utf16_encoded: Vec<_> = string
            .encode_utf16()
            .flat_map(|x| x.to_be_bytes())
            .collect();

        let obtained = lines_state.utf8_compute(utf16_encoded.as_slice()).output();

        assert_that!(obtained).is_equal_to(expected)
    }

    #[rstest]
    #[case("", 0)]
    #[case("Hello world\r", 1)]
    #[case("This is some \n long text", 0)]
    #[case("\r", 1)]
    #[case("\r\n", 1)]
    #[case(" \r", 1)]
    #[trace]
    fn utf8_cr_contains_the_expected_amount_of_line_breaks(
        #[with(LineBreak::CR)] lines_state: LinesState,
        #[case] string: &str,
        #[case] expected: usize,
    ) {
        let utf8_encoded = string.as_bytes();

        let obtained = lines_state.utf8_compute(utf8_encoded).output();

        assert_that!(obtained).is_equal_to(expected)
    }

    #[rstest]
    #[case("", 0)]
    #[case("Hello world\r", 1)]
    #[case("This is some \n long text", 0)]
    #[case("\r", 1)]
    #[case("\r\n", 1)]
    #[case(" \r", 1)]
    #[trace]
    fn utf16be_cr_contains_the_expected_amount_of_line_breaks(
        #[with(LineBreak::CR)] lines_state: LinesState,
        #[case] string: &str,
        #[case] expected: usize,
    ) {
        let utf16_encoded: Vec<_> = string
            .encode_utf16()
            .flat_map(|x| x.to_be_bytes())
            .collect();

        let obtained = lines_state.utf8_compute(utf16_encoded.as_slice()).output();

        assert_that!(obtained).is_equal_to(expected)
    }
}
