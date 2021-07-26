use crate::stats::parser_config::line_break::LineBreak::{CRLF, LF};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

const STR_CRLF: &str = "CRLF";
const STR_LF: &str = "LF";
#[derive(Clone)]
pub enum LineBreak {
    CRLF,
    LF,
}

impl Display for LineBreak {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let w = match self {
            CRLF => STR_CRLF,
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
            STR_CRLF => Ok(CRLF),
            _ => Err("Invalid line break type".to_string()),
        }
    }
}
