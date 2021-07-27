use clap::{load_yaml, App};
use cw::*;


fn main() {
    // Load clap for commandline utilities
    let yaml = load_yaml!("resources/cmdline-clap.yaml");
    let app = App::from(yaml).term_width(0);
    let matches = app.get_matches();

    // Program arguments
    let files = matches.values_of("FILES");
    let pretty_print = PrettyPrint::from_clap(&matches);
    let parser_config = Parser::from_clap(&matches);


    if let Some(files) = files {
        let num_threads = matches
            .value_of("threads")
            .map(|x| x.parse().unwrap_or(1))
            .unwrap_or(1);
        if num_threads == 1 {
            singlethread_files(files, pretty_print, &parser_config)
        } else if num_threads > 1 {
            multithread(files, pretty_print, num_threads, &parser_config)
        } else {
            eprintln!("Invalid threadcount");
            std::process::exit(1);
        }
    } else {
        singlethread_stdin(pretty_print, &parser_config);
    }
}
