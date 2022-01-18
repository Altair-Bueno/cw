use clap::{Parser,ArgGroup};
use libcw::config::{Encoding, LineBreak};

#[derive(Parser,Debug)]
#[clap(author, version, about, long_about = None)]
#[clap(
    group(
        ArgGroup::new("input")
            .required(false)
            .args(&["from-stdin", "files"])
    )
)]
pub struct Config {
    /// Read file paths from stdin
    ///
    /// When this flag is enabled, cw will treat each line in stdin as a file path
    //
    /// Example:
    ///     $ cat list_files.txt | cw --from-stdin
    #[clap(long)]
    pub from_stdin:bool,

    /// Counts line jumps
    #[clap(short,long)]
    pub lines: bool,

    /// Counts characters
    #[clap(short,long = "chars")]
    pub characters:bool,

    /// Counts words.
    ///
    /// cw count words as defined on ISSPACE(3)
    #[clap(short,long)]
    pub words: bool,

    /// Counts the number of bytes
    #[clap(short,long)]
    pub bytes:bool,

    /// Shows the longest line size
    #[clap(short= 'L',long)]
    pub line_length:bool,

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
    #[clap(short,long,default_value_t)]
    pub newline:LineBreak,

    /// Character encoding expected
    ///
    /// The file encoding used on these files. Posible values are UTF8 and UTF16
    /// Both UTF16 variants, Little Endian and Big Endian are supported out of
    /// the box
    #[clap(short,long,default_value_t)]
    pub encoding:Encoding,

    /// List of input files to analyze
    ///
    /// If no file is provided, cw will default to stdin input. Conflicts with
    /// `from-stdin` option
    #[clap()]
    pub files: Vec<String>,
}