use enum_utils::FromStr;

#[derive(Debug, Clone, Copy, Default, FromStr)]
pub enum Encoding {
    #[default]
    UTF8,
    UTF16,
    UTF16LE,
}

#[derive(Debug, Clone, Copy, Default, FromStr)]
pub enum LineBreak {
    #[default]
    LF,
    CR,
}

impl From<LineBreak> for u8 {
    fn from(b: LineBreak) -> Self {
        match b {
            LineBreak::LF => b'\n',
            LineBreak::CR => b'\r',
        }
    }
}

