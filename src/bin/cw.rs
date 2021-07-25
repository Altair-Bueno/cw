use clap::{App, ErrorKind, load_yaml};

use cw::commandline::Cwargs;
use cw::commandline::utilities::*;

fn main() {
    // Load clap for commandline utilities
    let yaml = load_yaml!("resources/cmdline-clap.yaml");
    let app = App::from(yaml);
    let matches = app.get_matches();

    // Program arguments
    let files = matches.values_of("files");
    let args = Cwargs::from_clap(&matches);

    if let Some(files) = files {
        let num_threads = matches.value_of("threads");
        match num_threads.map(|x| x.parse()) {
            None => singlethread_files(files, args),
            Some(Ok(x)) if x > 1 => multithread(files, args, x),
            Some(Ok(x)) if x == 1 => singlethread_files(files, args),
            Some(_) => {
                let message = format!(
                    "{} is not a valid number. Must be >=1",
                    num_threads.unwrap()
                );
                clap::Error::with_description(message, ErrorKind::InvalidValue).exit()
            }
        };
    } else {
        singlethread_stdio(args);
    }
}
