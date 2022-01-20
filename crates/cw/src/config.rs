use clap::{ArgGroup, Parser};

use libcw::config::{Encoding, LineBreak};
use libcw::Parser as CwParser;

#[derive(Parser, Debug, Clone)]
#[clap(author, version, about, long_about = None)]
#[clap(
group(
ArgGroup::new("input")
.required(false)
.args(& ["from-stdin", "files"])
)
)]
pub struct Config {
    /// Read file paths from stdin
    ///
    /// When this flag is enabled, cw will treat each line in stdin as a file path
    /// Example:
    ///     $ cat list_files.txt | cw --from-stdin
    #[clap(long, verbatim_doc_comment)]
    pub from_stdin: bool,

    /// Enables line count
    #[clap(short, long, verbatim_doc_comment)]
    pub lines: bool,

    /// Enables character count
    #[clap(short, long = "chars", verbatim_doc_comment)]
    pub characters: bool,

    /// Enables word count as defined on ISSPACE(3)
    #[clap(short, long, verbatim_doc_comment)]
    pub words: bool,

    /// Enables byte count
    #[clap(short, long, verbatim_doc_comment)]
    pub bytes: bool,

    /// Shows the longest line size
    #[clap(short = 'L', long, verbatim_doc_comment)]
    pub line_length: bool,

    /// Use multithreading
    ///
    /// cw ships with both tokio's async runtimes. By default, it cw will run
    /// using a single thread, but multithreading can be enabled for parsing
    /// more files at a time
    #[clap(long)]
    pub multithread: bool,

    /// Linebreak to use
    ///
    /// The kind of line break cw will search for. It can be LF '\n' or CR '\r'.
    /// For Windows' CRLF files either should work fine
    #[clap(short, long, default_value_t)]
    pub newline: LineBreak,

    /// Character encoding expected
    ///
    /// The file encoding used on these files. Posible values are UTF8 and UTF16
    /// Both UTF16 variants, Little Endian and Big Endian, are supported
    #[clap(short, long, default_value_t)]
    pub encoding: Encoding,

    /// List of input files to analyze
    ///
    /// If no file is provided, cw will default to stdin input. Conflicts with
    /// `from-stdin` option
    #[clap()]
    pub files: Vec<String>,
}

impl From<Config> for CwParser {
    fn from(config: Config) -> Self {
        let Config {
            lines,
            characters,
            words,
            bytes,
            line_length,
            newline,
            encoding,
            ..
        } = config;
        let is_custom = [lines, characters, words, bytes, line_length].contains(&true);

        if is_custom {
            CwParser::new(
                encoding,
                newline,
                lines,
                words,
                characters,
                bytes,
                line_length,
            )
        } else {
            CwParser::new(encoding, newline, true, true, false, true, false)
        }
    }
}