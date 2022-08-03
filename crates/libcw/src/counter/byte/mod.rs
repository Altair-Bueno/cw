#[cfg(feature="tower")]
mod service;
#[cfg(feature="tower")]
pub use service::*;

use super::Counter;

#[derive(Debug, Default, Clone)]
pub struct ByteCounterState {
    count: usize,
}

impl ByteCounterState {
    pub fn new() -> Self {
        Default::default()
    }
}

#[derive(Debug, Default, Clone)]
pub struct ByteCounter;

impl ByteCounter {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Counter<&[u8]> for ByteCounter {
    type State = ByteCounterState;
    type Output = usize;

    fn parse(&self, input: &[u8], mut state: Self::State) -> Self::State {
        state.count += input.len();
        state
    }

    fn terminate(&self, state: Self::State) -> usize {
        state.count
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::*;
    use speculoos::assert_that;

    #[fixture]
    pub fn state() -> ByteCounterState {
        ByteCounterState::default()
    }

    #[fixture]
    pub fn counter() -> ByteCounter {
        ByteCounter::new()
    }

    #[rstest]
    #[case("", 0)]
    #[case("Hello world", 11)]
    #[case("ñ", 2)]
    #[case("\r", 1)]
    #[case("❤", 3)]
    #[trace]
    fn utf8_has_the_expected_bytesize(
        state: ByteCounterState,
        counter: ByteCounter,
        #[case] input: &str,
        #[case] expected: usize,
    ) {
        let input = input.as_bytes();
        let state = counter.parse(input, state);
        let obtained = counter.terminate(state);

        assert_that!(obtained).is_equal_to(expected)
    }
}
