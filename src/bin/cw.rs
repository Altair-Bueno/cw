use clap::{load_yaml, App};
use cw::util::*;
use cw::exec_jobs::*;

fn main() {
    // Load clap for commandline utilities
    let yaml = load_yaml!("resources/cmdline-clap.yaml");
    let app = App::from(yaml);
    let matches = app.get_matches();

    // Files to proccess
    let files = matches.values_of("FILES");
    // Setup parser
    let parser = parser_from_clap(&matches);

    if let Some(files) = files {
        let num_threads = matches
            .value_of("threads")
            .map(|x| x.parse())
            .unwrap_or(Ok(1))
            .unwrap_or(1);
        match num_threads {
            1 => singlethread_files(files, &parser),
            x if x > 1 => multithread(files, &parser, x),
            _ => {
                eprintln!("Invalid threadcount");
                std::process::exit(1);
            }
        }
    } else {
        singlethread_stdin(&parser);
    }
}
