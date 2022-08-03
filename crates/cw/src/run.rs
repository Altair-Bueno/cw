use crate::print::JsonPrinter;
use crate::stats::Stats;
use anymap::AnyMap;
use eyre::Result;
use libcw::counter::line::LineCounter;
use libcw::counter::word::WordCounter;
use libcw::counter::Collapse;
use libcw::counter::{byte::ByteCounter, Counter};
use tokio_stream::Stream;
use tokio_stream::StreamExt;
use tower::{layer::util::Identity, ServiceBuilder};

use crate::print::{self, Printer};
use crate::{config::Config, util};

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
        let files = util::stdin_to_path_stream().await;
        //TODO
        todo!()
    } else if files.is_empty() {
        // Process stdin
        let mut stream = stdin(counter, state)
            .await
            .map(|x| x.map(TryFrom::try_from).map(Result::unwrap))
            .map(|x| ("STDIN".to_owned(), x));

        while let Some(message) = stream.next().await {
            printer.print(message).await;
        }
    } else {
        // File list provided as arguments
        tokio_stream::iter(files.into_iter());
        //TODO
        todo!()
    };

    printer.terminate().await;
    Ok(())
}

async fn stdin<C, S, O>(counter: C, state: S) -> impl Stream<Item = std::io::Result<AnyMap>>
where
    C: Counter<State = S, Output = O>,
    S: 'static,
    O: Collapse<AnyMap>,
{
    let collapsable = AnyMap::new();
    let reader = util::stdin_to_bufread().await;
    let result = util::count_bufreader(reader, counter, state, collapsable).await;

    tokio_stream::iter([result].into_iter())
}
