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

    if let Some(files) = files {
        match matches.value_of("threads").map(|x| x.parse()) {
            None => singlethread_files(files, args),
            Some(Ok(x)) if x > 1 => multithread(files, args, x),
            Some(Ok(x)) if x == 1 => singlethread_files(files, args),
            Some(Ok(x)) => eprintln!("{} is not a valid number. Must be >=1", x),
            Some(Err(err)) => eprintln!("{}", err),
        };
        std::process::exit(-1);
    } else {
        singlethread_stdio(args);
    }
}
