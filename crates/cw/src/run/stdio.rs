use tokio::io::BufReader;

use libcw::Parser;

use crate::message_writer::MessageWriter;

/// Async runner for stdio
pub async fn run_stdio<MessageWriterObject>(
    parser: Parser,
    output_writer: &mut Box<MessageWriterObject>,
) -> i32
where
    MessageWriterObject: MessageWriter + Send + Sync + ?Sized,
{
    let stdin = BufReader::new(tokio::io::stdin());
    let result = parser.process(stdin).await;
    let message = ("stdin".to_owned(), result);

    output_writer.message_received(message).await;
    output_writer.terminate().await
}
