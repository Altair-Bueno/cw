mod stats;
mod commandline;

use clap::{App, load_yaml};
use std::io::BufReader;
use std::fs::File;
use stats::Stats;
use crate::commandline::Arguments;

fn main() {
    // Load clap for commandline utilities
    let yaml = load_yaml!("../resources/cmdline-clap.yaml");
    let matches = App::from(yaml).get_matches();

    // Program arguments
    let files = matches.values_of("files");
    let args = Arguments::get_args(&matches);

    let exitcode =
    if let Some(files) = files {
        let (code, merged) = files
            .fold((0,Stats::new()),|(code,acc),file | {
                match use_file(file) {
                    Ok(stats) => {
                        let show = stats.show(&args);
                        println!("{} {}",show,file);
                        (code,acc.combine(&stats))
                    }
                    Err(err) => {
                        println!("{}",err);
                        (code + 1,acc)
                    }
                }
            });
        println!("{}", merged.show(&args));
        code
    } else {
        let stats_stdio = use_stdio();
        match stats_stdio {
            Ok(stats) => {
                let show = stats.show(&args);
                println!("{}",show);
                0
            }
            Err(err) => {
                println!("{}",err);
                -1
            }
        }
    };

    std::process::exit(exitcode);
}
fn use_file(f:&str) -> std::io::Result<Stats> {
    let reader = BufReader::new(
        File::open(f)?
    );
    let stats = Stats::from_file(Box::new(reader));

    stats
}

fn use_stdio() -> std::io::Result<Stats> {
    let reader = BufReader::new(std::io::stdin());
    let stats = Stats::from_file(Box::new(reader));

    stats
}
