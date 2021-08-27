//! cw (*count words*) is a faster alternative to classic GNU wc, written on pure
//! Rust. It provides the same tools as wc, but with some extras, such as
//! multithreading and multiple encoding support. cw also provides its core
//! functionality as a library called `libcw` that can target any arch with no
//! platform-specific code. The Rust compiler leverages great performance with
//! stupidly simple source code
//!
//! cw diferenciates itself from other wc clones by providing great defaults. cw
//! will **always** count characters using the provided encoding, and thus, always
//! providing the right count. Other word counters will provide, for example, wrong
//! max line length on UTF-8 encoded text
//!
//! To learn more about this proyect, visit it's [GitHub repo](https://github.com/Altair-Bueno/cw)
//!
use clap::{load_yaml, App, AppSettings};

use commandline::exec_jobs::*;
use commandline::util::parser_from_clap;

mod commandline;

fn main() {
    // Load clap for commandline utilities
    let yaml = load_yaml!("../resources/cmdline-clap.yaml");
    let app = App::from(yaml).setting(AppSettings::ColoredHelp);
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
            1 => singlethread_files(files, parser),
            x if x > 1 => multithread(files, parser, x),
            _ => {
                eprintln!("Invalid threadcount");
                std::process::exit(1);
            }
        }
    } else {
        singlethread_stdin(parser);
    }
}
