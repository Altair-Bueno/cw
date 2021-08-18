use std::fs::File;
use std::io::Write;
use std::io::{BufReader, BufWriter};
use std::result::Result::Ok;

use clap::Values;
use colored::Colorize;
use threads_pool::ThreadPool;

use libcw::Parser;
use libcw::Stats;

const TOTAL: &str = "total";

/// Multithread cw. Parses each file using a threadpool
pub fn multithread(files: Values, parser: &Parser, threads: usize) -> ! {
    // One thread for stdout
    let size = files.len();
    let pool = ThreadPool::new(threads);
    let (sender, reciver) = std::sync::mpsc::channel();

    // TODO use static references instead to avoid copy
    for f in files {
        let copy = sender.clone();
        let fclone = f.to_string();
        let parserclone: Parser = *parser;

        let _e = pool.execute(move || {
            let stats = from_file(fclone.as_str(), &parserclone);
            let _r = copy.send((fclone, stats));
            // eprintln!("{:?}",_r)
        });
        //eprintln!("{:?}",_e)
    }
    let exit_code = {
        let stdout = std::io::stdout();
        let stderr = std::io::stderr();
        let lock_stdout = stdout.lock();
        let lock_stderr = stderr.lock();
        let mut buff_stdout = BufWriter::new(lock_stdout);
        let mut buff_stderr = BufWriter::new(lock_stderr);

        let (code, acc) = (0..size).into_iter().zip(reciver.iter()).fold(
            (0, Stats::default()),
            |(code, acc), (_, (file, result))| match result {
                Ok(stats) => {
                    let _ = writeln!(buff_stdout, "{}{}", stats, file);
                    (code, acc.combine(stats))
                }
                Err(err) => {
                    let _ = writeln!(buff_stderr, "{}: {}", file, err);
                    (code + 1, acc)
                }
            },
        );

        if size > 1 {
            let _ = writeln!(
                buff_stdout,
                "{}{}",
                acc.to_string().as_str().green(),
                TOTAL.red()
            );
        }
        code
    }; // Drop locks and flush buffers
    std::process::exit(exit_code)
}

/// Singlethread for STDIN
pub fn singlethread_stdin(parser: &Parser) -> ! {
    let stats_stdio = from_stdin(parser);
    let exit_code = {
        let stdout = std::io::stdout();
        let stderr = std::io::stderr();
        let lock_stdout = stdout.lock();
        let lock_stderr = stderr.lock();
        let mut buff_stdout = BufWriter::new(lock_stdout);
        let mut buff_stderr = BufWriter::new(lock_stderr);

        let code = match stats_stdio {
            Ok(stats) => {
                let _ = writeln!(buff_stdout, "{}", stats);
                0
            }
            Err(err) => {
                let _ = writeln!(buff_stderr, "{}", err);
                1
            }
        };
        code
    }; // Drop locks and flush buffers
    std::process::exit(exit_code);
}

/// Single thread for FILES
pub fn singlethread_files(files: Values, parser: &Parser) -> ! {
    let size = files.len();
    let init = (0, Stats::default());

    let exit_code = {
        let stdout = std::io::stdout();
        let stderr = std::io::stderr();
        let lock_stdout = stdout.lock();
        let lock_stderr = stderr.lock();
        let mut buff_stdout = BufWriter::new(lock_stdout);
        let mut buff_stderr = BufWriter::new(lock_stderr);

        let (code, merged) = files.fold(init, |(code, acc), file| match from_file(file, parser) {
            Ok(stats) => {
                let _ = writeln!(buff_stdout, "{}{}", stats, file);
                (code, acc.combine(stats))
            }
            Err(err) => {
                let _ = writeln!(buff_stderr, "{}: {}", file, err);
                (code + 1, acc)
            }
        });

        if size > 1 {
            // Total files
            let _ = writeln!(
                buff_stdout,
                "{}{}",
                merged.to_string().as_str().green(),
                TOTAL.red()
            );
        }
        code
    }; // Drop locks and flush buffers
    std::process::exit(exit_code)
}

// Convenience functions
#[inline(always)]
fn from_file(f: &str, mode: &Parser) -> std::io::Result<Stats> {
    let file = File::open(f)?;
    let reader = BufReader::with_capacity(1024 * 32, file);
    mode.proccess(reader)
}

#[inline(always)]
fn from_stdin(mode: &Parser) -> std::io::Result<Stats> {
    let reader = BufReader::new(std::io::stdin());

    mode.proccess(reader)
}
