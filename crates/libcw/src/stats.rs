use std::{
    fmt::{Display, Formatter},
    ops::{Add, AddAssign},
};

#[derive(Debug, Clone, Copy, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Stats {
    pub lines: Option<usize>,
    pub words: Option<usize>,
    pub bytes: Option<usize>,
    pub chars: Option<usize>,
}
impl Add for Stats {
    type Output = Stats;

    fn add(self, rhs: Self) -> Self::Output {
        let lines = self.lines.zip(rhs.lines).map(|(a, b)| a + b);
        let words = self.words.zip(rhs.words).map(|(a, b)| a + b);
        let bytes = self.bytes.zip(rhs.bytes).map(|(a, b)| a + b);
        let chars = self.chars.zip(rhs.chars).map(|(a, b)| a + b);

        Self {
            lines,
            words,
            bytes,
            chars,
        }
    }
}
impl AddAssign for Stats {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
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
            self.lines, self.words, self.chars, self.bytes,
            //self.length,
        ];
        list.into_iter()
            .flatten()
            .map(|x| write!(f, "{}\t", x))
            .fold(Ok(()), |acc, n| acc.and(n))
    }
}
