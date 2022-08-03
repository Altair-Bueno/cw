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

use std::process::ExitCode;

use clap::Parser;

use config::Config;

mod config;
mod run;
mod util;
use run::run;

#[cfg_attr(feature = "mimalloc", global_allocator)]
#[cfg(feature = "mimalloc")]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[tokio::main(flavor = "current_thread")]
async fn current_thread_flavour(config: Config) -> u8 {
    run(config).await
}

#[tokio::main]
async fn multiple_threads_flavour(config: Config) -> u8 {
    run(config).await
}

fn main() -> ExitCode {
    let config: Config = Config::parse();

    if config.multithread {
        multiple_threads_flavour(config)
    } else {
        current_thread_flavour(config)
    }
    .into()
}
