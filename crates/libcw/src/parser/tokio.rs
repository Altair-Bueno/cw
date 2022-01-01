use crate::config::Encoding;
use crate::parser::BUFFER_SIZE;
use crate::state::traits::{compute::Compute, partial_state::PartialState};
use crate::stats::Stats;
use crate::Parser;
use tokio::io::{AsyncBufRead, AsyncBufReadExt, AsyncReadExt};

impl Parser {
    /// The process method takes in a [BufRead](tokio::io::BufRead) instance
    /// that is read for yielding results. If the BufRead instance cannot be
    /// read this will yield the corresponding error
    /// ```ignore
    /// # use libcw::Parser;
    /// # use libcw::config::{Encoding, LineBreak};
    /// use tokio::io::BufReader;
    /// use tokio::fs::File;
    ///
    /// # use std::io;
    /// # fn main() -> io::Result<()> {
    /// let parser = Parser::new(
    ///     Encoding::UTF8,
    ///     LineBreak::LF,
    ///     true,true,true,true,true
    /// );
    /// let read = BufReader::new(File::open("foo.txt").await?);
    /// let stats_from_read = parser.proccess(read).await;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn process<R>(&self, reader: R) -> std::io::Result<Stats>
    where
        R: AsyncReadExt + AsyncBufRead + Sized + Unpin,
    {
        match self.encoding {
            Encoding::UTF8 => self.utf8_process(reader).await,
            Encoding::UTF16 => self.utf16_process(reader).await,
        }
    }

    /// Runs over the tape at max speed reading utf8 encoded text
    pub(crate) async fn utf8_process<R>(&self, mut reader: R) -> std::io::Result<Stats>
    where
        R: AsyncReadExt + AsyncBufRead + Sized + Unpin,
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
    pub(crate) async fn utf16_process<R>(&self, mut reader: R) -> std::io::Result<Stats>
    where
        R: AsyncReadExt + AsyncBufRead + Sized + Unpin,
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
    pub(crate) async fn utf16_process_be<R>(&self, mut reader: R) -> std::io::Result<Stats>
    where
        R: AsyncReadExt + AsyncBufRead + Sized + Unpin,
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
    pub(crate) async fn utf16_process_le<R>(&self, mut reader: R) -> std::io::Result<Stats>
    where
        R: AsyncReadExt + AsyncBufRead + Sized + Unpin,
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
