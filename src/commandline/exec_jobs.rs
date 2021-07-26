use std::fs::File;
use std::io::BufReader;

use clap::{ErrorKind, Values};

use crate::commandline::PrettyPrint;
use crate::stats::automata::parser_config::AutomataConfig;
use crate::stats::Stats;
use std::result::Result::Ok;
use threads_pool::ThreadPool;

/// Multithread cw. Parses each file on individual files
pub fn multithread(files: Values, args: PrettyPrint, threads: usize, mode: &AutomataConfig) -> ! {
    // One thread for stdout
    let size = files.len();
    let pool = ThreadPool::new(threads);
    let (sender, reciver) = std::sync::mpsc::channel();

    // TODO use static references instead to avoid copy
    for f in files {
        let copy = sender.clone();
        let fclone = f.to_string();
        let modeclone: AutomataConfig = (*mode).clone();

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
pub fn singlethread_stdio(args: PrettyPrint, mode: &AutomataConfig) -> ! {
    let stats_stdio = from_stdio(mode);
     match stats_stdio {
        Ok(stats) => {
            let show = args.format_stats(&stats);
            println!("{}", show);
            std::process::exit(0);
        }
        Err(err) => clap::Error::with_description(err.to_string(),
                                                  ErrorKind::Io).exit(),
        // Todo too big. Use eprint instead
    }
}

/// Single thread proccess each file sequentialy. It does not instanciate a
/// thread pool, so startup is faster. Usefull when only reading one or two
/// files
pub fn singlethread_files(files: Values, args: PrettyPrint, mode: &AutomataConfig) -> ! {
    let size = files.len();
    let init = (0, Stats::default());

    let (code, merged) = files.fold(init, |(code, acc), file| {
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
        // Total files
        println!("\n{}\ttotal", args.format_stats(&merged));
    }
    std::process::exit(code)
}

fn from_file(f: &str, mode: &AutomataConfig) -> std::io::Result<Stats> {
    let file = File::open(f)?;
    let reader = BufReader::new(file);
    let stats = mode.proccess(Box::new(reader));

    stats
}

fn from_stdio(mode: &AutomataConfig) -> std::io::Result<Stats> {
    let reader = BufReader::new(std::io::stdin());
    let stats = mode.proccess(Box::new(reader));

    stats
}
