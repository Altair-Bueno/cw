use std::fs::File;
use std::io::BufReader;

use clap::{ErrorKind, Values};

use crate::commandline::pretty_print::PrettyPrint;
use crate::stats::parser::Parser;
use crate::stats::stats::Stats;
use std::result::Result::Ok;
use threads_pool::ThreadPool;

/// Multithread cw. Parses each file on individual files
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
            // eprintln!("{:?}",r)
        });
        //eprintln!("{:?}",e)
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

/// Proccess stdio using one single thread. Because stdio has an internal
/// lock, using more than one thread could impact performance
pub fn singlethread_stdio(args: PrettyPrint, mode: &Parser) -> ! {
    let stats_stdio = from_stdio(mode);
    match stats_stdio {
        Ok(stats) => {
            let show = args.print(&stats, "");
            println!("{}", show);

        }
        Err(err) =>eprintln!("{}",err),
    }
    std::process::exit(0);
}

/// Single thread proccess each file sequentialy. It does not instanciate a
/// thread pool, so startup is faster. Usefull when only reading one or two
/// files
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

fn from_file(f: &str, mode: &Parser) -> std::io::Result<Stats> {
    let file = File::open(f)?;
    let reader = BufReader::new(file);
    let stats = mode.proccess(Box::new(reader));

    stats
}

fn from_stdio(mode: &Parser) -> std::io::Result<Stats> {
    let reader = BufReader::new(std::io::stdin());
    let stats = mode.proccess(Box::new(reader));

    stats
}
