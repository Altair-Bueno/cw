use clap::{load_yaml, App};

use cw::commandline::exec_jobs::*;
use cw::commandline::PrettyPrint;
use cw::stats::automata::automata_config::AutomataConfig;

fn main() {
    // Load clap for commandline utilities
    let yaml = load_yaml!("resources/cmdline-clap.yaml");
    let app = App::from(yaml);
    let matches = app.get_matches();

    // Program arguments
    let files = matches.values_of("files");
    let pretty_print = PrettyPrint::from_clap(&matches);
    let parser_config = AutomataConfig::from_clap(&matches);

    // TODO better message
    println!("MODE: {}", parser_config);

    if let Some(files) = files {
        let num_threads = matches
            .value_of("threads")
            .map(|x| x.parse().unwrap_or(1))
            .unwrap_or(1);
        if num_threads == 1 {
            singlethread_files(files, pretty_print, &parser_config)
        } else if num_threads > 1 {
            multithread(files, pretty_print, num_threads, &parser_config)
        }
    } else {
        singlethread_stdio(pretty_print, &parser_config);
    }
}
