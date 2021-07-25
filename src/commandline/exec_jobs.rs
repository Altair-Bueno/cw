use std::fs::File;
use std::io::BufReader;

use clap::{ErrorKind, Values};

use crate::commandline::PrettyPrint;
use crate::stats::automata::Mode;
use crate::stats::Stats;

pub fn multithread(files: Values, args: PrettyPrint, threads: usize, mode: &Mode) -> ! {
    todo!()
}

pub fn singlethread_stdio(args: PrettyPrint, mode: &Mode) -> ! {
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

pub fn singlethread_files(files: Values, args: PrettyPrint, mode: &Mode) -> ! {
    let size = files.len();
    let (code, merged) = files.fold((0, Stats::default()), |(code, acc), file| {
        match from_file(file, mode) {
            Ok(stats) => {
                let show = args.format_stats(&stats);
                println!("{}\t{}", show, file);
                (code, acc + stats)
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

fn from_file(f: &str, mode: &Mode) -> std::io::Result<Stats> {
    let file = File::open(f)?;
    let reader = BufReader::new(file);
    let stats = mode.proccess(Box::new(reader));

    stats
}

fn from_stdio(mode: &Mode) -> std::io::Result<Stats> {
    let reader = BufReader::new(std::io::stdin());
    let stats = mode.proccess(Box::new(reader));

    stats
}
