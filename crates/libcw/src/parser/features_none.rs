use crate::config::Encoding;
use crate::parser::BUFFER_SIZE;
use crate::state::traits::compute::Compute;
use crate::state::traits::partial_state::PartialState;
use crate::{Parser, Stats};
use std::io::BufRead;

impl Parser {
    /// `process` exhausts a [`BufRead`](std::io::BufRead) object and returns
    /// the resulting [`Stats`](crate::Stats)
    ///
    /// # Errors
    ///
    /// Any error returned by the [`BufRead`](std::io::BufRead) object will be
    /// returned to the caller
    ///
    /// # Features
    ///
    /// If the `tokio` feature is enabled, the exposed API will be asynchronous
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
    pub fn process<R: BufRead + Sized>(&self, reader: R) -> std::io::Result<Stats> {
        match self.encoding {
            Encoding::UTF8 => self.utf8_process(reader),
            Encoding::UTF16 => self.utf16_process(reader),
        }
    }

    /// Runs over the tape at max speed reading utf8 encoded text
    pub(crate) fn utf8_process<R: BufRead + Sized>(&self, mut reader: R) -> std::io::Result<Stats> {
        let mut state = self.initial_state;
        let mut buff = [0; BUFFER_SIZE];
        loop {
            let read = reader.read(&mut buff)?;
            if read == 0 {
                return Ok(state.output());
            }
            state = state.utf8_compute(&buff[0..read]);
        }
    }

    /// Decides endianess and computes tape
    pub(crate) fn utf16_process<R: BufRead + Sized>(
        &self,
        mut reader: R,
    ) -> std::io::Result<Stats> {
        // TODO utf16 encoding on beta. Some test did not pass
        let buff = reader.fill_buf()?;
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

                self.utf16_process_le(reader).map(|x| x.combine(stats))
            } else if first == 0xFE && second == 0xFF {
                // Big endian
                let mut stats = self.initial_state.output();
                stats.set_bytes(Some(2));

                reader.consume(2);

                self.utf16_process_be(reader).map(|x| x.combine(stats))
            } else {
                // Assumed big endian
                self.utf16_process_be(reader)
            }
        }
    }
    pub(crate) fn utf16_process_be<R: BufRead + Sized>(
        &self,
        mut reader: R,
    ) -> std::io::Result<Stats> {
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
            read = reader.read(&mut buff[1..BUFFER_SIZE])?;

            if read == 0 {
                return Ok(state.output());
            } else {
                // Tape won't change. Non-mutable call
                state = state.utf16_compute(&buff[start..(read + 1)]);
            }
        }
    }
    pub(crate) fn utf16_process_le<R: BufRead + Sized>(
        &self,
        mut reader: R,
    ) -> std::io::Result<Stats> {
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
            read = reader.read(&mut buff[1..BUFFER_SIZE])?;

            for index in ((start + 1)..(read + 1)).step_by(2) {
                buff.swap(index, index - 1)
            }
            /*
            let mut index = start + 1;
            while index < read + 1 {
                buff.swap(index,index-1);
                index += 2;
            }*/

            if read == 0 {
                return Ok(state.output());
            } else {
                // Tape won't change. Non-mutable call
                state = state.utf16_compute(&buff[start..(read + 1)]);
            }
        }
    }
}
