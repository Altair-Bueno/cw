use std::cmp::max;
use std::fmt::{Display, Formatter};
use std::option::Option::Some;

/// Represents a set of stats. Is used as an output value for [Parser's proccess](crate::Parser::proccess)
/// method
///
/// # Suported stats list
/// - Number of lines
/// - Number of words
/// - Number of characters
/// - Number of bytes
/// - Max line legth
///
/// # Example
/// ```
/// use libcw::{Stats, Parser};
/// use libcw::config::{Encoding, LineBreak};
/// use std::io::BufRead;
///
/// let parser = Parser::new(
///     Encoding::UTF8,
///     LineBreak::LF,
///     true,true,true,true,true
/// );
/// let tape = b"Hello world";
/// let stats = parser.proccess(&tape[..]).unwrap();
/// assert_eq!(stats,Stats::new(Some(0),Some(2),Some(11),Some(11),Some(11)));
/// ```
///
#[derive(Debug, Eq, PartialEq)]
pub struct Stats {
    lines: Option<usize>,
    words: Option<usize>,
    characters: Option<usize>,
    bytes: Option<usize>,
    legth: Option<usize>,
    //colums: Colums,
}
impl Default for Stats {
    /// Default Stats for blank (`[]`) input is defined as:
    /// - 0 lines
    /// - 0 words
    /// - 0 characters
    /// - 0 bytes
    /// - max legth 0
    fn default() -> Self {
        Stats {
            lines: Some(0),
            words: Some(0),
            characters: Some(0),
            bytes: Some(0),
            legth: Some(0),
        }
    }
}

impl Stats {
    /// Creates a new Stats struct with the given information. This method
    /// is provided as a convenience for writing tests and should't be called
    /// unless there is a good reason for it
    pub fn new(
        lines: Option<usize>,
        words: Option<usize>,
        characters: Option<usize>,
        bytes: Option<usize>,
        legth: Option<usize>,
    ) -> Stats {
        Stats {
            lines,
            words,
            characters,
            bytes,
            legth,
        }
    }

    /// Combines two stats. Usefull for providing some combined results. If
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
            legth: combine_using(self.legth, s.legth, max),
        }
    }

    /// Returns the number of lines contained on this stats, if available
    pub fn lines(&self) -> Option<usize> {
        self.lines
    }

    /// Returns the number of words contained on this stats, if available
    pub fn words(&self) -> Option<usize> {
        self.words
    }

    /// Returns the number of characters contained on this stats, if available
    pub fn characters(&self) -> Option<usize> {
        self.characters
    }

    /// Returns the number of bytes contained on this stats, if available
    pub fn bytes(&self) -> Option<usize> {
        self.bytes
    }

    /// Returns the number max number of sequential characters between two line
    /// breaks (including start and end of file) contained on this stats, if
    /// available
    pub fn legth(&self) -> Option<usize> {
        self.legth
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
        if let Some(x) = self.lines {
            write!(f, "{}\t", x)?;
        }
        if let Some(x) = self.words {
            write!(f, "{}\t", x)?;
        }
        if let Some(x) = self.characters {
            write!(f, "{}\t", x)?;
        }
        if let Some(x) = self.bytes {
            write!(f, "{}\t", x)?;
        }
        if let Some(x) = self.legth {
            write!(f, "{}\t", x)?;
        }

        Ok(())
    }
}
