use std::option::Option::Some;
use std::result::Result::Ok;

use colored::Colorize;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt};
use tokio_stream::StreamExt;

use libcw::Parser;
use libcw::Stats;
use crate::Config;

const TOTAL: &str = "total";
const MAX_FILE_DESCRIPTORS: usize = 1024;

/// Selects the right async runner depending on the arguments provided
pub async fn run(config: Config, parser: Parser) -> i32 {
    if !config.files.is_empty() {
        let iterable = config.files.into_iter().map(Ok);
        let stream = tokio_stream::iter(iterable);
        run_files(stream, parser).await
    } else if config.from_stdin {
        let stdin = tokio::io::stdin();
        let buf = tokio::io::BufReader::new(stdin);
        let lines = tokio_stream::wrappers::LinesStream::new(buf.lines());
        run_files(lines, parser).await
    } else {
        run_stdio(parser).await
    }
}


/// Async runner for files
async fn run_files<S>(mut list: S, parser: Parser) -> i32
    where
        S: tokio_stream::Stream<Item=std::io::Result<String>> + Unpin + Send + Sync + 'static,
{
    let (s, mut r) = tokio::sync::mpsc::channel(MAX_FILE_DESCRIPTORS);
    let parser_clone = parser;
    tokio::spawn(async move {
        while let Some(Ok(path)) = list.next().await {
            let parser_clone = parser_clone;
            let closure = async move {
                let file = tokio::fs::File::open(&path).await;
                let response = match file {
                    Ok(file) => {
                        let mut buffer = tokio::io::BufReader::new(file);
                        let response = parser_clone.process(&mut buffer).await;
                        let _ = buffer.flush().await;
                        response
                    }
                    Err(err) => Err(err),
                };
                (path, response)
            };
            let handle = tokio::spawn(closure);
            let _ = s.send(handle).await;
        }
    });

    // stdio buffers
    let mut buff_stderr = tokio::io::BufWriter::new(tokio::io::stderr());
    let mut buff_stdout = tokio::io::BufWriter::new(tokio::io::stdout());
    let s = format!(
        "{} {}",
        parser.to_string().as_str().blue(),
        "File(s)\n".blue()
    );
    let _ = buff_stdout.write(s.as_bytes()).await;

    let mut code = 0;
    let mut merged = Stats::default();
    let mut canary: u8 = 0x2;
    while let Some(handle) = r.recv().await {
        if let Ok((path, result)) = handle.await {
            canary >>= 1;
            match result {
                Ok(x) => {
                    let s = format!("{}{}\n", x, path);
                    let _ = buff_stdout.write(s.as_bytes()).await;
                    merged = merged.combine(x);
                }
                Err(err) => {
                    let s = format!("{}: {}\n", path, err);
                    let _ = buff_stderr.write(s.as_bytes()).await;
                    code += 1;
                }
            }
        }
    }
    let _ = buff_stdout.flush().await;
    let _ = buff_stderr.flush().await;
    if canary == 0 {
        // Total files
        let s = format!("{}{}\n", merged.to_string().as_str().green(), TOTAL.green());
        let _ = buff_stdout.write(s.as_bytes()).await;
        let _ = buff_stdout.flush().await;
    }
    code
}

/// Async runner for stdio
async fn run_stdio(parser: Parser) -> i32 {
    let stdin = tokio::io::BufReader::new(tokio::io::stdin());

    let code = match parser.process(stdin).await {
        Ok(stats) => {
            println!("{}", parser.to_string().as_str().blue());
            println!("{}stdin", stats);
            0
        }
        Err(err) => {
            eprintln!("{}", err);
            1
        }
    };
    code
}
