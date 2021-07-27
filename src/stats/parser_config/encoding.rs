use crate::stats::parser_config::encoding::Encoding:: *;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

const STR_UTF8: &str = "UTF8";
const STR_UTF16: &str = "UTF16";

#[derive(Clone)]
pub enum Encoding {
    UTF8,
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
