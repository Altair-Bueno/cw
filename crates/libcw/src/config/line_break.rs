use std::fmt::{Display, Formatter};
use std::str::FromStr;

use crate::config::LineBreak::*;

const STR_CR: &str = "CR";
const STR_LF: &str = "LF";

/// Represents a set of supported line breaks for a [Parser](crate::Parser).
/// Currently, it supports:
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub enum LineBreak {
    /// Carriage return. Often used on old Macintosh systems
    /// - Unicode code: U+000D
    /// - Default: No
    CR,
    /// Line feed. Most common implementation of new line used on all POSIX
    /// systems such as macOS, Linux and FreeBSD
    /// - Unicode code: U+000A
    /// - Default: Yes
    LF,
}

impl LineBreak {
    /// Returns the byte used to represent this character
    pub fn get_separator(&self) -> u8 {
        match self {
            CR => b'\r',
            LF => b'\n',
        }
    }
}

impl Display for LineBreak {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let w = match self {
            CR => STR_CR,
            LF => STR_LF,
        };
        write!(f, "{}", w)
    }
}

impl Default for LineBreak {
    fn default() -> Self {
        LineBreak::LF
    }
}

impl FromStr for LineBreak {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            STR_LF => Ok(LF),
            STR_CR => Ok(CR),
            _ => Err("Invalid line break type".to_string()),
        }
    }
}
