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
use clap::Parser;

use config::Config;
use exec_jobs::*;
use libcw::Parser as CwParser;
use tokio::io::AsyncBufReadExt;

mod config;
mod exec_jobs;

#[cfg_attr(feature = "mimalloc", global_allocator)]
#[cfg(feature = "mimalloc")]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

fn config_to_parser(config: &Config) -> CwParser {
    let Config {
        lines,
        characters,
        words,
        bytes,
        line_length,
        newline,
        encoding,
        ..
    } = config;
    let is_custom = [lines, characters, words, bytes, line_length].contains(&&true);

    if is_custom {
        CwParser::new(
            *encoding,
            *newline,
            *lines,
            *words,
            *characters,
            *bytes,
            *line_length,
        )
    } else {
        CwParser::new(*encoding, *newline, true, true, false, true, false)
    }
}

#[tokio::main(flavor = "current_thread")]
async fn current_thread_flavour(config: Config, parser: CwParser) -> i32 {
    run(config, parser).await
}

#[tokio::main]
async fn multiple_threads_flavour(config: Config, parser: CwParser) -> i32 {
    run(config, parser).await
}

async fn run(config: Config, parser: CwParser) -> i32 {
    if !config.files.is_empty() {
        let iterable = config.files.into_iter().map(Ok);
        let stream = tokio_stream::iter(iterable);
        process_files(stream, parser).await
    } else if config.from_stdin {
        let stdin = tokio::io::stdin();
        let buf = tokio::io::BufReader::new(stdin);
        let lines = tokio_stream::wrappers::LinesStream::new(buf.lines());
        process_files(lines, parser).await
    } else {
        process_stdin(parser).await
    }
}

fn main() -> ! {
    let config: Config = Config::parse();
    let parser = config_to_parser(&config);
    let out_code = if config.multithread {
        multiple_threads_flavour(config, parser)
    } else {
        current_thread_flavour(config, parser)
    };
    std::process::exit(out_code)
}
