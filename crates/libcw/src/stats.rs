use std::cmp::max;
use std::fmt::{Display, Formatter};
use std::option::Option::Some;

#[cfg(feature = "serde")]
use serde::Serialize;

/// Represents a set of stats. Is used as an output value for
/// [Parser's process](crate::Parser::process) method
///
/// # Supported stats list
///
/// - Number of lines
/// - Number of words
/// - Number of characters
/// - Number of bytes
/// - Max line length
#[derive(Debug, Eq, PartialEq, Clone)]
#[cfg_attr(feature = "serde",derive(Serialize))]
pub struct Stats {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    lines: Option<usize>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    words: Option<usize>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    characters: Option<usize>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    bytes: Option<usize>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    length: Option<usize>,
    //columns: Columns,
}

impl Default for Stats {
    /// Default Stats for blank (`[]`) input is defined as:
    /// - 0 lines
    /// - 0 words
    /// - 0 characters
    /// - 0 bytes
    /// - max length 0
    fn default() -> Self {
        Stats {
            lines: Some(0),
            words: Some(0),
            characters: Some(0),
            bytes: Some(0),
            length: Some(0),
        }
    }
}

impl Display for Stats {
    /// Displays the contained stats using this format
    /// ```text
    /// lines\twords\tcharacters\tbytes\tlength\t
    /// ```
    /// If any value is missing (eg words is None), then said value and its
    /// right tab will be missing
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let list = [
            self.lines,
            self.words,
            self.characters,
            self.bytes,
            self.length,
        ];
        list.into_iter()
            .flatten()
            .map(|x| write!(f, "{}\t", x))
            .fold(Ok(()), |acc, n| acc.and(n))
    }
}

impl Stats {
    /// Creates a new `Stats` struct with the given stats.
    pub fn new(
        lines: Option<usize>,
        words: Option<usize>,
        characters: Option<usize>,
        bytes: Option<usize>,
        length: Option<usize>,
    ) -> Stats {
        Stats {
            lines,
            words,
            characters,
            bytes,
            length,
        }
    }

    /// Combines two stats. Useful for providing some combined results. If
    /// any of the combined stats has a missing value, the result will **also**
    /// have a missing value
    /// ```
    /// use libcw::Stats;
    /// let stats1 = Stats::new(Some(1),Some(2),Some(1),Some(10),Some(0));
    /// let stats2 = Stats::new(Some(8),Some(3),Some(4),None,Some(5));
    /// let result = Stats::new(Some(9),Some(5),Some(5),None,Some(5));
    /// assert_eq!(stats1.combine(stats2), result)
    /// ```
    pub fn combine(self, s: Stats) -> Stats {
        let combine_using = |a, b, f: fn(usize, usize) -> usize| match (a, b) {
            (Some(x), Some(y)) => Some(f(x, y)),
            _ => None,
        };

        Stats {
            lines: combine_using(self.lines, s.lines, std::ops::Add::add),
            words: combine_using(self.words, s.words, std::ops::Add::add),
            characters: combine_using(self.characters, s.characters, std::ops::Add::add),
            bytes: combine_using(self.bytes, s.bytes, std::ops::Add::add),
            length: combine_using(self.length, s.length, max),
        }
    }

    /// Returns the number of lines contained on these stats, if available
    pub fn lines(&self) -> Option<usize> {
        self.lines
    }

    /// Returns the number of words contained on these stats, if available
    pub fn words(&self) -> Option<usize> {
        self.words
    }

    /// Returns the number of characters contained on these stats, if available
    pub fn characters(&self) -> Option<usize> {
        self.characters
    }

    /// Returns the number of bytes contained on these stats, if available
    pub fn bytes(&self) -> Option<usize> {
        self.bytes
    }

    /// Returns the number max number of sequential characters between two line
    /// breaks (including start and end of file) contained on these stats, if
    /// available
    pub fn length(&self) -> Option<usize> {
        self.length
    }

    /// Changes the stored line count for these stats
    pub fn set_lines(&mut self, lines: Option<usize>) {
        self.lines = lines;
    }
    /// Changes the stored word count for these stats
    pub fn set_words(&mut self, words: Option<usize>) {
        self.words = words;
    }
    /// Changes the stored character count for these stats
    pub fn set_characters(&mut self, characters: Option<usize>) {
        self.characters = characters;
    }
    /// Changes the stored byte count for these stats
    pub fn set_bytes(&mut self, bytes: Option<usize>) {
        self.bytes = bytes;
    }
    /// Changes the stored byte count for these stats
    pub fn set_length(&mut self, legth: Option<usize>) {
        self.length = legth;
    }
}

#[cfg(all(test, feature = "serde"))]
mod serde_test {
    use rstest::*;
    use speculoos::assert_that;

    use crate::Stats;

    #[rstest]
    fn stats_generates_the_expected_json_output() {
        let expected = r#"{"lines":0,"words":10}"#.to_owned();
        let stats = Stats::new(Some(0), Some(10), None, None, None);

        let obtained = serde_json::to_string(&stats).unwrap();

        assert_that!(obtained).is_equal_to(expected)
    }
}