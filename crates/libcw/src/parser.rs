use std::fmt::{Display, Formatter};

use crate::config::Encoding;
use crate::config::LineBreak;
use crate::state::bytes_state::BytesState;
use crate::state::chars_state::CharState;
use crate::state::lines_state::LinesState;
use crate::state::max_length::MaxLengthState;
use crate::state::words_state::WordsState;
use crate::state::State;

#[cfg(any(feature = "sync", feature = "tokio"))]
use crate::{
    state::traits::{compute::Compute, partial_state::PartialState},
    stats::Stats,
};

cfg_if::cfg_if! {
if #[cfg(feature="tokio")] {
    use tokio::io::{AsyncBufRead, AsyncBufReadExt, AsyncReadExt};

    trait Reader: AsyncReadExt + AsyncBufRead + Sized + Unpin {}

    impl<T> Reader for T
    where T: AsyncReadExt + AsyncBufRead + Sized + Unpin
    {}

} else if #[cfg(feature="sync")] {
    use std::io::BufRead;

    trait Reader: BufRead {}

    impl<T> Reader for T
    where T: BufRead
    {}
}
}

#[allow(unused)]
const BUFFER_SIZE: usize = 16 * 1024; // 8KB

/// Parser is libcw's main component. It provides abstractions over the
/// different counters contained inside this crate. To learn more how to use
/// `Parser` read [`Parser::process`](crate::Parser::process)
///
/// The default `Parser` configuration will count **lines**, **words** and **characters**
#[derive(Default, Copy, Clone, Debug)]
pub struct Parser {
    initial_state: State,
    encoding: Encoding,
    linebreak: LineBreak,
}

impl Display for Parser {
    /// Displays the current configuration set-up for this Parser instance using
    /// this format
    ///
    /// ```text
    /// l\tw\tc\tb\tL\t
    /// ```
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{} {}",
            self.initial_state, self.encoding, self.linebreak
        )
    }
}

impl Parser {
    /// Creates a new parser instance with the given configuration. It's
    /// important to note that a Parser instance is **inmutable**. It can be
    /// used across threads without any kind of locks. Also,
    /// it can be combined with the [lazy_static](https://crates.io/crates/lazy_static)
    /// macro for sharing one single instance across different threads. Setting
    /// a Parser correctly is important: You only pay for what you need, meaning
    /// It'll only compute for the stats you asked for and thus taking the least
    /// amount of time to return
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

        if lines {
            initial_state.set_lines_state(Some(LinesState::new(linebreak)))
        };

        if words {
            initial_state.set_words_state(Some(WordsState::new()))
        };

        if chars {
            initial_state.set_char_state(Some(CharState::new()))
        };

        if bytes {
            initial_state.set_bytes_state(Some(BytesState::new()))
        };

        if max_length {
            initial_state.set_max_length_state(Some(MaxLengthState::new(linebreak)))
        };

        Parser {
            initial_state,
            encoding,
            linebreak,
        }
    }
}

#[cfg(any(feature = "tokio", feature = "sync"))]
impl Parser {
    /// `process` exhausts a [`AsyncReadExt`](tokio::io::AsyncReadExt) object and returns
    /// the resulting [`Stats`](crate::Stats)
    ///
    /// # Errors
    ///
    /// Any error returned by the [`AsyncReadExt`](tokio::io::AsyncReadExt) object will be
    /// returned to the caller
    ///
    /// # Example
    ///
    /// ```rust
    /// use libcw::Parser;
    /// use libcw::config::{Encoding, LineBreak};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let parser = Parser::default();
    ///     let data = b"Some large byte stream";
    ///     let stats = parser.process(data.as_slice()).await.unwrap();
    ///
    ///     assert_eq!(Some(data.len()),stats.bytes())
    /// }
    /// ```
    #[maybe_async::async_impl]
    pub async fn process<R>(&self, reader: R) -> std::io::Result<Stats>
    where
        R: AsyncReadExt + AsyncBufRead + Sized + Unpin,
    {
        match self.encoding {
            Encoding::UTF8 => self.utf8_process(reader).await,
            Encoding::UTF16 => self.utf16_process(reader).await,
        }
    }

    /// `process` exhausts a [`BufRead`](std::io::BufRead) object and returns
    /// the resulting [`Stats`](crate::Stats)
    ///
    /// # Errors
    ///
    /// Any error returned by the [`BufRead`](std::io::BufRead) object will be
    /// returned to the caller
    ///
    /// # Example
    ///
    /// ```rust
    /// use libcw::Parser;
    /// use libcw::config::{Encoding, LineBreak};
    ///
    /// let parser = Parser::default();
    /// let data = b"Some large byte stream";
    /// let stats = parser.process(data.as_slice()).unwrap();
    ///
    /// assert_eq!(Some(data.len()),stats.bytes())
    /// ```
    #[maybe_async::sync_impl]
    pub fn process<R: std::io::BufRead + Sized>(&self, reader: R) -> std::io::Result<Stats> {
        match self.encoding {
            Encoding::UTF8 => self.utf8_process(reader),
            Encoding::UTF16 => self.utf16_process(reader),
        }
    }

    /// Runs over the tape at max speed reading utf8 encoded text
    #[maybe_async::maybe_async]
    async fn utf8_process<R>(&self, mut reader: R) -> std::io::Result<Stats>
    where
        R: Reader,
    {
        let mut state = self.initial_state;
        let mut buff = [0; BUFFER_SIZE];
        loop {
            let read = reader.read(&mut buff).await?;
            if read == 0 {
                return Ok(state.output());
            }
            state = state.utf8_compute(&buff[0..read]);
        }
    }

    /// Decides endianess and computes tape
    #[maybe_async::maybe_async]
    async fn utf16_process<R>(&self, mut reader: R) -> std::io::Result<Stats>
    where
        R: Reader,
    {
        // TODO utf16 encoding on beta. Some test did not pass
        let buff = reader.fill_buf().await?;
        if buff.len() < 2 {
            // Not enough
            let mut out = self.initial_state.output();
            out.set_bytes(Some(buff.len()));
            Ok(out)
        } else {
            let first = buff[0];
            let second = buff[1];

            if first == 0xFF && second == 0xFE {
                // Little endian
                let mut stats = self.initial_state.output();
                stats.set_bytes(Some(2));

                reader.consume(2);

                self.utf16_process_le(reader)
                    .await
                    .map(|x| x.combine(stats))
            } else if first == 0xFE && second == 0xFF {
                // Big endian
                let mut stats = self.initial_state.output();
                stats.set_bytes(Some(2));

                reader.consume(2);

                self.utf16_process_be(reader)
                    .await
                    .map(|x| x.combine(stats))
            } else {
                // Assumed big endian
                self.utf16_process_be(reader).await
            }
        }
    }

    #[maybe_async::maybe_async]
    async fn utf16_process_be<R>(&self, mut reader: R) -> std::io::Result<Stats>
    where
        R: Reader,
    {
        let mut state = self.initial_state;
        let mut buff = [0; BUFFER_SIZE];

        let mut read = 0;
        loop {
            let start = if read % 2 != 0 {
                // Put last one the first
                buff[0] = buff[read];
                0
            } else {
                // Ignore the first element
                1
            };
            // [_,Some,Some,Some,Some...,BUFFER_SIZE]
            read = reader.read(&mut buff[1..BUFFER_SIZE]).await?;

            if read == 0 {
                return Ok(state.output());
            } else {
                // Tape wont change. Non mutable call
                state = state.utf16_compute(&buff[start..(read + 1)]);
            }
        }
    }
    #[maybe_async::maybe_async]
    async fn utf16_process_le<R>(&self, mut reader: R) -> std::io::Result<Stats>
    where
        R: Reader,
    {
        let mut state = self.initial_state;
        let mut buff = [0; BUFFER_SIZE];

        let mut read = 0;
        loop {
            let start = if read % 2 != 0 {
                // Put last one the first
                buff[0] = buff[read];
                0
            } else {
                // Ignore the first element
                1
            };
            // [_,Some,Some,Some,Some...,BUFFER_SIZE]
            read = reader.read(&mut buff[1..BUFFER_SIZE]).await?;

            for index in ((start + 1)..(read + 1)).step_by(2) {
                buff.swap(index, index - 1)
            }

            if read == 0 {
                return Ok(state.output());
            } else {
                // Tape won't change. Non mutable call
                state = state.utf16_compute(&buff[start..(read + 1)]);
            }
        }
    }
}
