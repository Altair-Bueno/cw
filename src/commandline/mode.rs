
pub enum Encoding {
    UTF8,
    ASCII,
}
pub enum LineBreak {
    CRLF,
    LF,
}

pub struct Mode {
    encoding: Encoding,
    line_break:LineBreak,
}

impl Mode {
    pub fn new (encoding:Encoding,line_break:LineBreak) -> Mode {
        Mode { encoding, line_break }
    }
}