use std::fmt::{Display, Formatter};
use std::str::FromStr;

use crate::config::Encoding::*;

const STR_UTF8: &str = "UTF8";
const STR_UTF16: &str = "UTF16";

/// Represents a set of supported encodings for a [Parser](crate::Parser).
/// Currently, cw only works with UTF8/ASCII (default) encoding, but support
/// will be increased in the future
#[derive(Copy, Clone)]
pub enum Encoding {
    /// UTF-8 encoded text. It's the default setting for parsers
    UTF8,
    /// UTF-16 encoded text. Currently it is not stable
    UTF16,
}

impl Display for Encoding {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let w = match self {
            UTF8 => STR_UTF8,
            // UTF16 => STR_ASCII,
            UTF16 => STR_UTF16,
        };
        write!(f, "{}", w)
    }
}
impl Default for Encoding {
    fn default() -> Self {
        Encoding::UTF8
    }
}

impl FromStr for Encoding {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            STR_UTF8 => Ok(UTF8),
            STR_UTF16 => Ok(UTF16),
            _ => Err("Invalid encoding".to_string()),
        }
    }
}
