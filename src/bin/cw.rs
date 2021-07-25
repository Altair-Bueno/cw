use clap::{load_yaml, App};

use cw::commandline::exec_jobs::*;
use cw::commandline::PrettyPrint;
use cw::stats::automata::automata::Mode;

fn main() {
    // Load clap for commandline utilities
    let yaml = load_yaml!("resources/cmdline-clap.yaml");
    let app = App::from(yaml);
    let matches = app.get_matches();

    // Program arguments
    let files = matches.values_of("files");
    let args = PrettyPrint::from_clap(&matches);
    let mode = Mode::new(
        matches
            .value_of("encoding")
            .map(|x| x.parse().unwrap_or_default())
            .unwrap_or_default(),
        matches
            .value_of("break")
            .map(|x| x.parse().unwrap_or_default())
            .unwrap_or_default(),
    );
    // TODO better message
    println!("MODE: {}", mode);

    if let Some(files) = files {
        let num_threads = matches
            .value_of("threads")
            .map(|x| x.parse().unwrap_or(1))
            .unwrap_or(1);
        if num_threads == 1 {
            singlethread_files(files, args, &mode)
        } else if num_threads > 1 {
            multithread(files, args, num_threads, &mode)
        }
    } else {
        singlethread_stdio(args, &mode);
    }
}
