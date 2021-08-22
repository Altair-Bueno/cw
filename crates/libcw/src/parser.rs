use std::fmt::{Display, Formatter};
use std::io::BufRead;

use crate::config::Encoding;
use crate::config::LineBreak;
use crate::state::bytes_state::BytesState;
use crate::state::chars_state::CharState;
use crate::state::lines_state::LinesState;
use crate::state::max_length::MaxLengthState;
use crate::state::traits::{compute::Compute, partial_state::PartialState};
use crate::state::words_state::WordsState;
use crate::state::State;
use crate::stats::Stats;

const BUFFER_SIZE: usize = 16 * 1024; // 8KB

/// Parser is libcw's main component. It provides abstractions over the
/// different counters contained inside this crate. It has an easy to use
/// interface API that results on a [Stats](crate::Stats) instance with the
/// yielded results
///
/// # Default search configuration
///
/// - lines
/// - words
/// - bytes
///
/// # Example
///
/// ```no_run
/// # use libcw::Parser;
/// # use libcw::config::{Encoding, LineBreak};
/// # use std::io::BufReader;
/// # use std::fs::File;
/// # use std::io;
/// # fn main() -> io::Result<()> {
/// let parser = Parser::new(
///     Encoding::UTF8,
///     LineBreak::LF,
///     true,true,true,true,true
/// );
/// let read = BufReader::new(File::open("foo.txt")?);
/// let stats_from_read = parser.proccess(read);
/// # Ok(())
/// # }
/// ```
#[derive(Default, Copy, Clone, Debug)]
pub struct Parser {
    initial_state: State,
}

impl Parser {
    /// Creates a new parser instance with the given configuration. It's
    /// important to note that a Parser instance is **inmutable**. It can be
    /// used across threads without any kind of locks. Also,
    /// it can be combined with the [lazy_static](https://crates.io/crates/lazy_static)
    /// macro for sharing one single instance across different threads. Setting
    /// a Parser correctly is important: You only pay for what you need, meaning
    /// It'll only compute for the stats you asked for and thus taking the least
    /// amout of time to return
    pub fn new(
        encoding: Encoding,
        linebreak: LineBreak,
        lines: bool,
        words: bool,
        chars: bool,
        bytes: bool,
        max_length: bool,
    ) -> Parser {
        let mut initial_state = State::new();

        // todo encoding not used right now
        if lines {
            initial_state.set_lines_state(Some(LinesState::new(linebreak)))
        };

        if words {
            initial_state.set_words_state(Some(WordsState::new()))
        };

        if chars {
            initial_state.set_char_state(Some(CharState::new(encoding)))
        };

        if bytes {
            initial_state.set_bytes_state(Some(BytesState::new()))
        };

        if max_length {
            initial_state.set_max_length_state(Some(MaxLengthState::new(linebreak, encoding)))
        };

        Parser { initial_state }
    }

    /// The proccess method takes in a [BufRead](std::io::BufRead) instance
    /// that is read for yielding results. If the BufRead instance canotn be
    /// read this will yield the corresponding error
    /// ```no_run
    /// # use libcw::Parser;
    /// # use libcw::config::{Encoding, LineBreak};
    /// # use std::io::BufReader;
    /// # use std::fs::File;
    /// # use std::io;
    /// # fn main() -> io::Result<()> {
    /// let parser = Parser::new(
    ///     Encoding::UTF8,
    ///     LineBreak::LF,
    ///     true,true,true,true,true
    /// );
    /// let read = BufReader::new(File::open("foo.txt")?);
    /// let stats_from_read = parser.proccess(read);
    /// # Ok(())
    /// # }
    /// ```
    pub fn proccess<R: BufRead + Sized>(&self, mut reader: R) -> std::io::Result<Stats> {
        let mut state = self.initial_state;
        let mut buff = [0; BUFFER_SIZE];
        // TODO detect endianness if UTF16 https://en.wikipedia.org/wiki/UTF-16
        loop {
            let read = reader.read(&mut buff)?;
            if read == 0 {
                return Ok(state.output());
            }
            state = state.compute(&buff[0..read]);
        }
    }
}
impl Display for Parser {
    /// Displays the current configuration set-up for this Parser instance using
    /// this format
    ///
    /// ```text
    /// l(type)\tw\tc\tb\tL\t
    /// ```
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.initial_state.fmt(f)
    }
}
