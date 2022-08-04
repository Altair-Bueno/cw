mod files;
mod stdin;
use crate::print::JsonPrinter;
use eyre::Result;
use libcw::counter::byte::ByteCounter;
use libcw::counter::line::LineCounter;
use libcw::counter::word::WordCounter;
use libcw::{Stats, StatsBuilder};
use tower::{layer::util::Identity, ServiceBuilder};

use crate::print::Printer;
use crate::{config::Config, util};

use stdin::count_stdin;

pub async fn run(config: Config) -> Result<()> {
    let Config {
        from_stdin,
        files,
        json,
        ..
    } = config;

    // Setup middleware
    let counter = ServiceBuilder::new()
        .layer(ByteCounter::new())
        .layer(LineCounter::new(Default::default()))
        .layer(WordCounter::new())
        .service(Identity::new());

    let zero: usize = 0;
    let stats = StatsBuilder::default()
        .bytes(zero)
        .lines(zero)
        .words(zero)
        .build()
        .unwrap();
    let state = Default::default();


        // Setup printer
    let printer: Box<dyn Printer + Send + Sync> = if json {
        Box::new(JsonPrinter::new(stats.clone()))
    } else {
        todo!()
    };

    // Hook up to service
    let runner = if from_stdin {
        // File list provided by stdin
        let files = util::stdin_to_path_stream().await;
        todo!()
    } else if files.is_empty() {
        // Process stdin
        count_stdin(counter, stats, state, printer).await?;
        todo!()
    } else {
        // File list provided as arguments
        let files = tokio_stream::iter(files.into_iter());
        //TODO
        todo!()
    };

    Ok(())
}
