use clap::{load_yaml, App, Values};
use cw::{commandline::Cwargs, stats::Stats};
use std::{fs::File, io::BufReader};

fn main() {
    // Load clap for commandline utilities
    let yaml = load_yaml!("../resources/cmdline-clap.yaml");
    let matches = App::from(yaml).get_matches();

    // Program arguments
    let files = matches.values_of("files");
    let args = Cwargs::from_clap(&matches);
    // u8 unsigned.
    // TODO threading could be 0
    let threading: usize = matches
        .value_of("threads")
        .map(|x| x.parse().unwrap_or(1))
        .unwrap_or(1);

    if let Some(files) = files {
        if threading == 1 {
            singlethread_files(files, args);
        } else {
            multithread(files, args, threading);
        }
    } else {
        singlethread_stdio(args);
    }
}
fn multithread(files: Values, args: Cwargs, threads: usize) -> ! {
    todo!()
}
fn singlethread_stdio(args: Cwargs) -> ! {
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

fn singlethread_files(files: Values, args: Cwargs) -> ! {
    let (code, merged) = files.fold((0, Stats::default()), |(code, acc), file| {
        match from_file(file) {
            Ok(stats) => {
                let show = args.pretty_print_stats(&stats);
                println!("{}\t{}", show, file);
                (code, acc + stats)
            }
            Err(err) => {
                println!("{}", err);
                (code + 1, acc)
            }
        }
    });

    println!("{} total", args.pretty_print_stats(&merged));
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
