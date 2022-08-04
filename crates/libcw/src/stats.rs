use derive_builder::Builder;
use std::ops::{Add, AddAssign};

#[derive(Debug, Clone, Copy, Default, Builder)]
#[cfg_attr(feature="serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Stats {
    #[builder(setter(into, strip_option), default)]
    pub lines: Option<usize>,
    #[builder(setter(into, strip_option), default)]
    pub words: Option<usize>,
    #[builder(setter(into, strip_option), default)]
    pub bytes: Option<usize>,
}
impl Add for Stats {
    type Output = Stats;

    fn add(self, rhs: Self) -> Self::Output {
        let lines = self.lines.zip(rhs.lines).map(|(a,b)|a+b);
        let words = self.words.zip(rhs.words).map(|(a,b)|a+b);
        let bytes = self.bytes.zip(rhs.bytes).map(|(a,b)|a+b);

        Self { lines, words, bytes }
    }
}
impl AddAssign for Stats {
    fn add_assign(&mut self, rhs: Self) {
        *self = self.clone() + rhs
    }
}
