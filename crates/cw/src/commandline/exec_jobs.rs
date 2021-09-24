use std::result::Result::Ok;
use std::iter::Iterator;

use tokio::io::AsyncWriteExt;
use colored::Colorize;

use libcw::Parser;
use libcw::Stats;
use std::option::Option::Some;
use tokio::task::JoinHandle;

const TOTAL: &str = "total";
const MAX_FILE_DESCRIPTORS : usize = 512;

pub async fn process_files(v:Vec<&str>, parser: Parser) -> ! {
    // TODO Remove vectors. They allocate memory on the heap that may lead to
    // cache miss
    let size = v.len();

    let (s, mut r) = tokio::sync::mpsc::channel(MAX_FILE_DESCRIPTORS);
    let cloned_vec = v.iter().map(ToString::to_string).collect::<Vec<String>>();

    tokio::spawn(async move {
        for e in cloned_vec {
            let handle = tokio::spawn(async move {
                match tokio::fs::File::open(e).await {
                    Ok(file) => {
                        let reader = tokio::io::BufReader::new(file);
                        parser.proccess(reader).await
                    }
                    Err(err) => Err(err)
                }
            });
            let send_result = s.send(handle).await;
            if send_result.is_err() {
                break
            }
        }
    });

    // stdio buffers
    let mut buff_stderr = tokio::io::BufWriter::new(tokio::io::stderr());
    let mut buff_stdout = tokio::io::BufWriter::new(tokio::io::stdout());
    let s = format!("{} {}",parser.to_string().as_str().blue(),"File(s)\n".blue());
    let _ = buff_stdout.write(s.as_bytes()).await;


    let (code, merged) = {
        let mut iter = v.iter();
        let mut code = 0;
        let mut stats = Stats::default();
        while let Some(next) = r.recv().await {
            let next = next.await;
            // Unwrap here is safe
            let file = iter.next().unwrap();
            match next {
                Ok(Ok(x)) => {
                    let s = format!("{}{}\n", x, file);
                    let _ = buff_stdout.write(s.as_bytes()).await;
                    stats = stats.combine(x);
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
        (code,stats)
    };
    let _ = buff_stderr.flush().await;
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