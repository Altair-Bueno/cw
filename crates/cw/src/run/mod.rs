use std::result::Result::Ok;

use tokio::io::AsyncBufReadExt;

use libcw::Parser;

use crate::Config;
use crate::message_writer::default::DefaultMessageWriter;
use crate::message_writer::json::JsonMessageWriter;
use crate::message_writer::MessageWriter;
use crate::run::files::run_files;
use crate::run::stdio::run_stdio;

mod stdio;
mod files;

/// Selects the right async runner depending on the arguments provided
pub async fn run(config: Config, parser: Parser) -> i32 {
    let Config { json, files, from_stdin, .. } = config;

    let mut output_writer: Box<dyn MessageWriter + Send + Sync> = match json {
        true => Box::new(JsonMessageWriter::init()),
        false => Box::new(DefaultMessageWriter::init(parser.clone()).await),
    };

    if !files.is_empty() {
        let iterable = files.into_iter().map(Ok);
        let stream = tokio_stream::iter(iterable);

        run_files(stream, parser, &mut output_writer).await
    } else if from_stdin {
        let stdin = tokio::io::stdin();
        let buf = tokio::io::BufReader::new(stdin);
        let lines = tokio_stream::wrappers::LinesStream::new(buf.lines());

        run_files(lines, parser, &mut output_writer).await
    } else {
        run_stdio(parser, &mut output_writer).await
    }
}
