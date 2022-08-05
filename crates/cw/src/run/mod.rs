mod files;
mod stdin;
use std::ops::Not;
use std::path::PathBuf;

use crate::print::{JsonPrinter, StdoutPrinter};
use crate::statefull_counter::{Eat, StatsCounter};
use eyre::Result;
use libcw::counter::byte::ByteCounter;
use libcw::counter::char::CharCounter;
use libcw::counter::line::LineCounter;

use libcw::counter::word::WordCounter;
use libcw::Stats;
use tower::layer::util::Identity;
use tower::Layer;

use crate::print::Printer;
use crate::{config::Config, util};
use stdin::count_stdin;
use tokio_stream::StreamExt;

use self::files::count_files;

fn setup(config: &Config) -> (Vec<Box<dyn Eat>>, Stats) {
    let mut stats = Stats::default();
    let mut eaters = Vec::with_capacity(10);

    if config.bytes {
        let counter = ByteCounter::new().layer(Identity::new());
        eaters.push(Box::new(StatsCounter::new(counter, Default::default())) as _);
        stats.bytes = Some(0);
    }

    if config.lines {
        let counter = LineCounter::new(config.newline).layer(Identity::new());
        eaters.push(Box::new(StatsCounter::new(counter, Default::default())) as _);
        stats.lines = Some(0);
    }

    if config.words {
        let counter = WordCounter::new().layer(Identity::new());
        eaters.push(Box::new(StatsCounter::new(counter, Default::default())) as _);
        stats.words = Some(0);
    }

    if config.characters {
        let counter = CharCounter::new().layer(Identity::new());
        eaters.push(Box::new(StatsCounter::new(counter, Default::default())) as _);
        stats.chars = Some(0);
    }

    (eaters, stats)
}

pub async fn run(mut config: Config) -> Result<()> {
    if [
        config.lines,
        config.characters,
        config.bytes,
        config.line_length,
    ]
    .iter()
    .all(Not::not)
    {
        config.characters = true;
        config.words = true;
        config.lines = true;
    }
    let (eaters, stats) = setup(&config);
    let Config {
        from_stdin,
        files,
        json,
        ..
    } = config;

    // Setup printer
    let printer: Box<dyn Printer + Send + Sync> = if json {
        Box::new(JsonPrinter::new(stats))
    } else {
        Box::new(StdoutPrinter::new(stats))
    };

    if from_stdin {
        // File list provided by stdin
        let files = util::stdin_to_path_stream().await
            .map(|x|x.map(|x| PathBuf::from(x)));
        count_files(files, eaters, stats, printer).await?;
    } else if files.is_empty() {
        // Process stdin
        count_stdin(eaters, stats, printer).await?;
    } else {
        // File list provided as arguments
        let files = tokio_stream::iter(files.into_iter())
            .map(Ok);
        count_files(files, eaters, stats, printer).await?;
    };

    Ok(())
}
