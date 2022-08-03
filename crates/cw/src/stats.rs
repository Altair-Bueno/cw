use std::ops::Add;

use anymap::AnyMap;
use derive_builder::Builder;
use libcw::counter::{byte::Bytes, line::Lines, word::Words};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Builder, Default)]
pub struct Stats {
    lines: Lines,
    words: Words,
    bytes: Bytes,
}

impl Add for Stats {
    type Output = Stats;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            lines: Lines(*self.lines + *rhs.lines),
            words: Words(*self.words + *rhs.words),
            bytes: Bytes(*self.bytes + *rhs.bytes),
        }
    }
}

impl TryFrom<AnyMap> for Stats {
    type Error = ();

    fn try_from(value: AnyMap) -> Result<Self, Self::Error> {
        fn inner(value: AnyMap) -> Option<Stats> {
            let lines = value.get::<Lines>()?.clone();
            let words = value.get::<Words>()?.clone();
            let bytes = value.get::<Bytes>()?.clone();

            Some(Stats {lines, words, bytes})
        }
        inner(value).ok_or(())
    }
}

