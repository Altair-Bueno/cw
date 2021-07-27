use std::fs::File;
use std::io::BufReader;

use clap::Values;

use crate::commandline::pretty_print::PrettyPrint;
use crate::stats::parser::Parser;
use crate::stats::stats::Stats;
use std::result::Result::Ok;
use threads_pool::ThreadPool;

/// Multithread cw. Parses each file using a threadpool
pub fn multithread(files: Values, args: PrettyPrint, threads: usize, mode: &Parser) -> ! {
    // One thread for stdout
    let size = files.len();
    let pool = ThreadPool::new(threads);
    let (sender, reciver) = std::sync::mpsc::channel();

    // TODO use static references instead to avoid copy
    for f in files {
        let copy = sender.clone();
        let fclone = f.to_string();
        let modeclone: Parser = (*mode).clone();

        let _e = pool.execute(move || {
            let stats = from_file(fclone.as_str(), &modeclone);
            let _r = copy.send((fclone, stats));
            // eprintln!("{:?}",_r)
        });
        //eprintln!("{:?}",_e)
    }

    let (code, acc) = (0..size).into_iter().zip(reciver.iter()).fold(
        (0, Stats::default()),
        |(code, acc), (_, (file, result))| match result {
            Ok(stats) => {
                let show = args.print(&stats, &file[..]);
                println!("{}", show);
                (code, acc.combine(stats))
            }
            Err(err) => {
                eprintln!("{}: {}", file, err);
                (code + 1, acc)
            }
        },
    );

    if size > 1 {
        println!("{}", args.print(&acc, "total"));
    }
    std::process::exit(code)
}

/// Singlethread for STDIN
pub fn singlethread_stdin(args: PrettyPrint, mode: &Parser) -> ! {
    let stats_stdio = from_stdin(mode);
    let code = match stats_stdio {
        Ok(stats) => {
            let show = args.print(&stats, "");
            println!("{}", show);
            0
        }
        Err(err) => {
            eprintln!("{}", err);
            1
        }
    };
    std::process::exit(code);
}

/// Single thread for FILES
pub fn singlethread_files(files: Values, args: PrettyPrint, mode: &Parser) -> ! {
    let size = files.len();
    let init = (0, Stats::default());

    let (code, merged) = files.fold(init, |(code, acc), file| match from_file(file, mode) {
        Ok(stats) => {
            let show = args.print(&stats, file);
            println!("{}", show);
            (code, acc.combine(stats))
        }
        Err(err) => {
            eprintln!("{}: {}", file, err);
            (code + 1, acc)
        }
    });

    if size > 1 {
        // Total files
        println!("\n{}", args.print(&merged, "total"));
    }
    std::process::exit(code)
}

// Convenience functions
fn from_file(f: &str, mode: &Parser) -> std::io::Result<Stats> {
    let file = File::open(f)?;
    let reader = BufReader::new(file);

    mode.proccess(reader)
}

fn from_stdin(mode: &Parser) -> std::io::Result<Stats> {
    let reader = BufReader::new(std::io::stdin());

    mode.proccess(reader)
}
