mod stats;

use clap::{App, load_yaml};
use std::io::BufReader;
use std::fs::File;
use stats::Stats;

fn main() {
    // Load clap for commandline utilities
    let yaml = load_yaml!("../resources/cmdline-clap.yaml");
    let matches = App::from(yaml).get_matches();

    // Program arguments
    let files = matches.values_of("files");
    let lines = matches.is_present("lines");
    let words = false;
    let characters = false;
    let bytes = false;

    let exitcode =
    if let Some(files) = files {
        files.fold(0,|code,file | {
            match use_file(file) {
                Ok(stats) => {
                    let show = stats.show(lines,words,characters,bytes);
                    println!("{}:{}",show,file);
                    code
                }
                Err(err) => {
                    println!("{}",err);
                    code + 1
                }
            }
        })
    } else {
        let stats_stdio = use_stdio();
        match stats_stdio {
            Ok(stats) => {
                let show = stats.show(lines,words,characters,bytes);
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
