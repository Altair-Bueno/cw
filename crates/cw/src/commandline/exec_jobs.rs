use std::result::Result::Ok;

use colored::Colorize;
use tokio::io::AsyncWriteExt;

use libcw::Parser;
use libcw::Stats;
use std::option::Option::Some;
use tokio_stream::StreamExt;

const TOTAL: &str = "total";
const MAX_FILE_DESCRIPTORS: usize = 512;

pub async fn process_files<S>(mut list: S, parser: Parser) -> !
where
    S: tokio_stream::Stream<Item = std::io::Result<String>> + Unpin + Send + Sync + 'static
{
    let (s, mut r) = tokio::sync::mpsc::channel(MAX_FILE_DESCRIPTORS);
    tokio::spawn(async move {
        while let Some(Ok(next)) = list.next().await {
            let cloned_string = next.clone();
            let handle = tokio::spawn(async move {
                match tokio::fs::File::open(cloned_string).await {
                    Ok(file) => {
                        let reader = tokio::io::BufReader::new(file);
                        parser.proccess(reader).await
                    }
                    Err(err) => Err(err),
                }
            });
            if let Err(_) = s.send((handle,next)).await {
                break;
            }
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
    let mut canary:u8 = 0x2;
    while let Some ((handle,file)) = r.recv().await{
        canary = canary >> 1;
        match handle.await {
            Ok(Ok(x)) => {
                let s = format!("{}{}\n", x, file);
                let _ = buff_stdout.write(s.as_bytes()).await;
                merged = merged.combine(x);
            }
            Ok(Err(err)) => {
                let s = format!("{}: {}\n", file, err);
                let _ = buff_stderr.write(s.as_bytes()).await;
                code += 1;
            }
            Err(err) => {
                let s = format!("{}: {}\n", file, err);
                let _ = buff_stderr.write(s.as_bytes()).await;
                code += 1;
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
    std::process::exit(code)
}

pub async fn proccess_stdin(parser: Parser) -> ! {
    let stdin = tokio::io::BufReader::new(tokio::io::stdin());

    let code = match parser.proccess(stdin).await {
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
    std::process::exit(code)
}
