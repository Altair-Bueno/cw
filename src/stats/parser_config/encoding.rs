use crate::stats::parser_config::encoding::Encoding:: UTF8;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

const STR_UTF8: &str = "UTF8";
// const STR_ASCII: &str = "ASCII";

#[derive(Clone)]
pub enum Encoding {
    UTF8,
    // ASCII,
}
impl Display for Encoding {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let w = match self {
            UTF8 => STR_UTF8,
            // ASCII => STR_ASCII,
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
            // STR_ASCII => Ok(ASCII),
            _ => Err("Invalid encoding".to_string()),
        }
    }
}
