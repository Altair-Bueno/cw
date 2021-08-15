use clap::{load_yaml, App};
use cw::*;

fn main() {
    // Load clap for commandline utilities
    let yaml = load_yaml!("resources/cmdline-clap.yaml");
    let app = App::from(yaml).term_width(0);
    let matches = app.get_matches();

    // Program arguments
    let files = matches.values_of("FILES");
    let parser = Parser::from_clap(&matches);


    if let Some(files) = files {
        let num_threads = matches
            .value_of("threads")
            .map(|x| x.parse())
            .unwrap_or(Ok(1))
            .unwrap_or(1);
        match num_threads {
            1 => singlethread_files(files,&parser),
            x if x> 1 => todo!(),// multithread(files,parser,x),
            _ => {
                eprintln!("Invalid threadcount");
                std::process::exit(1);
            }
        }
    } else {
        singlethread_stdin(&parser);
    }
}
