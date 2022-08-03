#[cfg(feature = "clap")]
use clap::ValueEnum;
use enum_utils::FromStr;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "clap", derive(ValueEnum))]
#[derive(Debug, Clone, Copy, Default, FromStr)]
pub enum Encoding {
    #[default]
    UTF8,
    UTF16,
    UTF16LE,
}

impl Display for Encoding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let e = match self {
            Encoding::UTF8 => "UTF8",
            Encoding::UTF16 => "UTF16",
            Encoding::UTF16LE => "UTF16LE",
        };
        write!(f, "{e}")
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "clap", derive(ValueEnum))]
#[derive(Debug, Clone, Copy, Default, FromStr)]
pub enum LineBreak {
    #[default]
    LF,
    CR,
}

impl Display for LineBreak {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let e = match self {
            LineBreak::LF => "LF",
            LineBreak::CR => "CR",
        };
        write!(f, "{e}")
    }
}

impl From<LineBreak> for u8 {
    fn from(b: LineBreak) -> Self {
        match b {
            LineBreak::LF => b'\n',
            LineBreak::CR => b'\r',
        }
    }
}
