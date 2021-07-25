use std::fs::File;
use std::io::BufReader;

use clap::{ErrorKind, Values};

use crate::commandline::PrettyPrint;
use crate::stats::automata::file_style::FileStyle;
use crate::stats::Stats;
use std::result::Result::Ok;
use threads_pool::ThreadPool;

/// Multithread cw. Parses each file on individual files
pub fn multithread(files: Values, args: PrettyPrint, threads: usize, mode: &FileStyle) -> ! {
    // One thread for stdout
    let size = files.len();

    let pool = ThreadPool::new(threads);
    let (sender, reciver) = std::sync::mpsc::channel();
    for f in files {
        let copy = sender.clone();
        let fclone = f.to_string();
        let modeclone: FileStyle = (*mode).clone();

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
                let show = args.format_stats(&stats);
                println!("{}\t{}", show, file);
                (code, acc.combine(stats))
            }
            Err(err) => {
                eprintln!("{}: {}", file, err);
                (code + 1, acc)
            }
        },
    );

    if size > 1 {
        println!("{}\ttotal", args.format_stats(&acc));
    }
    std::process::exit(code)
}

/// Proccess stdio using one single thread. Because stdio has an internal
/// lock, using more than one thread could impact performance
pub fn singlethread_stdio(args: PrettyPrint, mode: &FileStyle) -> ! {
    let stats_stdio = from_stdio(mode);
    let code = match stats_stdio {
        Ok(stats) => {
            let show = args.format_stats(&stats);
            println!("{}", show);
            0
        }
        Err(err) => clap::Error::with_description(err.to_string(), ErrorKind::Io).exit(),
    };
    std::process::exit(code);
}

/// Single thread proccess each file sequentialy. It does not instanciate a
/// thread pool, so startup is faster. Usefull when only reading one or two
/// files
pub fn singlethread_files(files: Values, args: PrettyPrint, mode: &FileStyle) -> ! {
    let size = files.len();
    let (code, merged) = files.fold((0, Stats::default()), |(code, acc), file| {
        match from_file(file, mode) {
            Ok(stats) => {
                let show = args.format_stats(&stats);
                println!("{}\t{}", show, file);
                (code, acc.combine(stats))
            }
            Err(err) => {
                eprintln!("{}: {}", file, err);
                (code + 1, acc)
            }
        }
    });

    if size > 1 {
        println!("{}\ttotal", args.format_stats(&merged));
    }
    std::process::exit(code)
}

fn from_file(f: &str, mode: &FileStyle) -> std::io::Result<Stats> {
    let file = File::open(f)?;
    let reader = BufReader::new(file);
    let stats = mode.proccess(Box::new(reader));

    stats
}

fn from_stdio(mode: &FileStyle) -> std::io::Result<Stats> {
    let reader = BufReader::new(std::io::stdin());
    let stats = mode.proccess(Box::new(reader));

    stats
}
