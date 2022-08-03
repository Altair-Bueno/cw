mod stdin;
mod files;
use crate::print::JsonPrinter;
use eyre::Result;
use libcw::counter::byte::ByteCounter;
use libcw::counter::line::LineCounter;
use libcw::counter::word::WordCounter;
use tower::{layer::util::Identity, ServiceBuilder};

use crate::print::Printer;
use crate::{config::Config, util};

use stdin::count_stdin;

pub async fn run(config: Config) -> Result<()> {
    let counter = ServiceBuilder::new()
        .layer(ByteCounter::new())
        .layer(LineCounter::new(Default::default()))
        .layer(WordCounter::new())
        .service(Identity::new());
    let state = Default::default();

    let Config {
        from_stdin,
        files,
        json,
        ..
    } = config;

    let mut printer: Box<dyn Printer + Send + Sync> = if json {
        Box::new(JsonPrinter::new())
    } else {
        todo!()
    };

    printer.begin().await;
    if from_stdin {
        // File list provided by stdin
        //TODO
        let files = util::stdin_to_path_stream().await;
        todo!()
    } else if files.is_empty() {
        // Process stdin
        count_stdin(counter, state, printer.as_mut()).await?;
    } else {
        // File list provided as arguments
        tokio_stream::iter(files.into_iter());
        //TODO
        todo!()
    };

    printer.terminate().await;
    Ok(())
}
