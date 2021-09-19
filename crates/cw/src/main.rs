//! cw (*count words*) is a faster alternative to classic GNU wc, written on pure
//! Rust. It provides the same tools as wc, but with a more friendly interface
//! and multiple encoding support. cw also provides its core
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
use clap::{load_yaml, App, AppSettings, Values};

use commandline::exec_jobs::*;
use commandline::util::parser_from_clap;
use libcw::Parser;

mod commandline;

//#[tokio::main(flavor = "current_thread")]
fn main() {
    // Load clap for commandline utilities
    let yaml = load_yaml!("../resources/cmdline-clap.yaml");
    let app = App::from(yaml).setting(AppSettings::ColoredHelp);
    let matches = app.get_matches();
    let parser = parser_from_clap(&matches);
    // Files to proccess
    let files = matches.values_of("FILES");
    if matches.is_present("multithread") {
        multiple_threads_flavour(files,parser)
    } else {
        current_thread_flavour(files,parser)
    }
}

#[tokio::main(flavor="current_thread")]
async fn current_thread_flavour(files:Option<Values>,parser:Parser) -> ! {
    if let Some(values) = files {
        let v: Vec<&str> = values.collect();
        process_files(v, parser).await
    } else {
        proccess_stdin(parser).await
    }
}
#[tokio::main]
async fn multiple_threads_flavour(files:Option<Values>,parser:Parser) -> ! {
    if let Some(values) = files {
        let v: Vec<&str> = values.collect();
        process_files(v, parser).await
    } else {
        proccess_stdin(parser).await
    }
}