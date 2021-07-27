use crate::stats::parser_config::line_break::LineBreak::*;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

const STR_CR: &str = "CR";
const STR_LF: &str = "LF";
#[derive(Clone)]
pub enum LineBreak {
    CR,
    LF,
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
