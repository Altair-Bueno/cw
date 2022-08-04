use std::ops::{Add, AddAssign};
#[derive(Debug, Clone, Copy, Default)]
#[cfg_attr(feature="serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Stats {
    pub lines: usize,
    pub words: usize,
    pub bytes: usize,
}
impl Add for Stats {
    type Output = Stats;

    fn add(self, rhs: Self) -> Self::Output {
        Stats {
            lines: self.lines + rhs.lines,
            words: self.words + rhs.words,
            bytes: self.bytes + rhs.bytes,
        }
    }
}
impl AddAssign for Stats {
    fn add_assign(&mut self, rhs: Self) {
        *self = self.clone() + rhs
    }
}
