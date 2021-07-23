use clap::Values;

use std::fs::File;
use std::io::BufReader;

use crate::commandline::Cwargs;
use crate::stats::Stats;

pub fn multithread(files: Values, args: Cwargs, threads: usize) -> ! {
    todo!()
}

pub fn singlethread_stdio(args: Cwargs) -> ! {
    let stats_stdio = from_stdio();
    let code = match stats_stdio {
        Ok(stats) => {
            let show = args.pretty_print_stats(&stats);
            println!("{}", show);
            0
        }
        Err(err) => {
            println!("{}", err);
            -1
        }
    };
    std::process::exit(code);
}

pub fn singlethread_files(files: Values, args: Cwargs) -> ! {
    let size = files.len();
    let (code, merged) = files.fold((0, Stats::default()), |(code, acc), file| {
        match from_file(file) {
            Ok(stats) => {
                let show = args.pretty_print_stats(&stats);
                println!("{}\t{}", show, file);
                (code, acc + stats)
            }
            Err(err) => {
                println!("{}: {}", file, err);
                (code + 1, acc)
            }
        }
    });

    if size > 1 {
        println!("{}\ttotal", args.pretty_print_stats(&merged));
    }
    std::process::exit(code)
}

fn from_file(f: &str) -> std::io::Result<Stats> {
    let file = File::open(f)?;
    let reader = BufReader::new(file);
    let stats = Stats::from_file(Box::new(reader));

    stats
}

fn from_stdio() -> std::io::Result<Stats> {
    let reader = BufReader::new(std::io::stdin());
    let stats = Stats::from_file(Box::new(reader));

    stats
}
