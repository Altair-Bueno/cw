use tokio::io::AsyncWriteExt;
use tokio_stream::{Stream, StreamExt};

use libcw::{Parser, Stats};

use crate::message_writer::MessageWriter;

const MAX_CONCURRENT_FILE_DESCRIPTORS: usize = 1024;

/// Async runner for files
pub async fn run_files<InputStream, MessageWriterObject>(
    mut list: InputStream,
    parser: Parser,
    output_writer: &mut Box<MessageWriterObject>,
) -> i32
    where
        InputStream: Stream<Item=std::io::Result<String>> + Unpin + Send + Sync + 'static,
        MessageWriterObject: MessageWriter + Send + Sync + ?Sized,
{
    let (sender, mut receiver) = tokio::sync::mpsc::channel(MAX_CONCURRENT_FILE_DESCRIPTORS);

    tokio::spawn(async move {
        while let Some(Ok(path)) = list.next().await {
            let async_block = async move { process_path(parser, path).await };
            let handle = tokio::spawn(async_block);
            let _ = sender.send(handle).await;
        }
    });

    while let Some(handle) = receiver.recv().await {
        if let Ok(message) = handle.await {
            output_writer.message_received(message).await;
        }
    }
    output_writer.terminate().await
}

async fn process_path(parser: Parser, path: String) -> (String, std::io::Result<Stats>) {
    async fn closure(parser: Parser, path: &String) -> std::io::Result<Stats> {
        let file = tokio::fs::File::open(&path).await?;
        let mut buffer = tokio::io::BufReader::new(file);
        let response = parser.process(&mut buffer).await;
        // Forces Tokio to close the file descriptor
        let _ = buffer.flush().await;
        response
    }

    let response = closure(parser, &path).await;
    (path, response)
}

