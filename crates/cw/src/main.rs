//! cw (*count words*) is a faster alternative to classic GNU wc, written on pure
//! Rust. It provides the same tools as wc, but with a more friendly interface
//! and multiple encoding support. cw also provides its core
//! functionality as a library called `libcw` that can target any arch with no
//! platform-specific code. The Rust compiler leverages great performance with
//! stupidly simple source code
//!
//! cw differentiates itself from other wc clones by providing great defaults. cw
//! will **always** count characters using the provided encoding, and thus, always
//! providing the right count. Other word counters will provide, for example, wrong
//! max line length on UTF-8 encoded text
//!
//! To learn more about this project, visit it's [GitHub repo](https://github.com/Altair-Bueno/cw)
//!
use clap::{load_yaml, App, AppSettings, ArgMatches};

use commandline::exec_jobs::*;
use commandline::util::parser_from_clap;
use libcw::Parser;
use tokio::io::AsyncBufReadExt;

mod commandline;


#[cfg_attr(feature = "mimalloc",global_allocator)]
#[cfg(feature = "mimalloc")]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

fn main() -> ! {
    // Load clap for commandline utilities
    let yaml = load_yaml!("../resources/cmdline-clap.yaml");
    let app = App::from(yaml).setting(AppSettings::ColoredHelp);
    let matches = app.get_matches();
    let parser = parser_from_clap(&matches);
    // Files to process
    let code = if matches.is_present("multithread") {
        multiple_threads_flavour(matches,parser)
    } else {
        current_thread_flavour(matches,parser)
    };
    std::process::exit(code)
}

#[tokio::main(flavor="current_thread")]
async fn current_thread_flavour(matches:ArgMatches,parser:Parser) -> i32 {
    run(matches,parser).await
}

#[tokio::main]
async fn multiple_threads_flavour(matches:ArgMatches,parser:Parser) -> i32 {
    run(matches,parser).await
}

async fn run(matches:ArgMatches<'_>,parser:Parser) -> i32 {
    if let Some(values) = matches.values_of("FILES") {
        let vec = values
            .map(ToString::to_string)
            .map(Ok).collect::<Vec<std::io::Result<String>>>();
        let stream = tokio_stream::iter(vec);
        process_files(stream, parser).await
    } else if matches.is_present("from-stdin") {
        let stdin = tokio::io::stdin();
        let buf = tokio::io::BufReader::new(stdin);
        let lines = tokio_stream::wrappers::LinesStream::new(buf.lines());
        process_files(lines,parser).await
    } else {
        process_stdin(parser).await
    }
}