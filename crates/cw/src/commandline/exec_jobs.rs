use std::result::Result::Ok;
use std::iter::Iterator;

use tokio::io::AsyncWriteExt;
use colored::Colorize;

use libcw::Parser;
use libcw::Stats;

const TOTAL: &str = "total";

pub async fn process_files(v:Vec<&str>, parser: Parser) -> ! {
    // TODO Remove vectors. They allocate memory on the heap that may lead to
    // cache miss
    let size = v.len();
    let mut buff_stderr = tokio::io::BufWriter::new(tokio::io::stderr());
    let mut buff_stdout = tokio::io::BufWriter::new(tokio::io::stdout());
    let s = format!("{} {}",parser.to_string().as_str().blue(),"File(s)\n".blue());
    let _ = buff_stdout.write(s.as_bytes()).await;
    let mut tasks = Vec::with_capacity(size);

    for next in v.iter() {
        let path = next.to_string();
        let parser = parser.clone();
        let handle = tokio::spawn(async move {
            let file = tokio::fs::File::open(&path).await?;
            let buff = tokio::io::BufReader::new(file);
            parser.proccess(buff).await
        });
        tasks.push(handle)
    }
    let (code,merged) = {
        let mut code = 0;
        let mut stats = Stats::default();
        for (task,file) in tasks.into_iter().zip(v) {
            match task.await {
                Ok(Ok(x)) => {
                    let s = format!("{}{}\n", x, file);
                    let _ = buff_stdout.write(s.as_bytes()).await;
                    stats = stats.combine(x);
                },
                Ok(Err(err)) => {
                    let s = format!("{}: {}\n", file, err);
                    let _ = buff_stderr.write(s.as_bytes()).await;
                    code +=1;
                },
                // Althought it should always recive a result from `tokio`, i
                // prefer to handle this error
                #[cfg(not(debug_assertions))]
                _ => {},
                #[cfg(debug_assertions)]
                some => {
                    let s = format!("{:?}",some);
                    let _ = buff_stderr.write(s.as_bytes()).await;
                },
            }
        }
        (code,stats)
    };

    if size > 1 {
        // Total files
        let s = format!(
            "{}{}\n",
            merged.to_string().as_str().green(),
            TOTAL.green()
        );
        let _ = buff_stdout.write(s.as_bytes()).await;
    }
    let _ = buff_stdout.flush().await;
    let _ = buff_stderr.flush().await;
    std::process::exit(code)
}

pub async fn proccess_stdin(parser:Parser) -> ! {
    let stdin = tokio::io::BufReader::new(tokio::io::stdin());

    let code = match parser.proccess(stdin).await {
        Ok(stats) => {
            println!("{}",parser.to_string().as_str().blue());
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