use clap::{load_yaml, App};

use cw::commandline::utilities::*;
use cw::commandline::Cwargs;

fn main() {
    // Load clap for commandline utilities
    let yaml = load_yaml!("resources/cmdline-clap.yaml");
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