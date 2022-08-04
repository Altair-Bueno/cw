use libcw::{
    counter::{Collapse, Counter},
    Stats,
};
use std::io::Result;
use tokio::io::{AsyncBufRead, AsyncWriteExt};
use tokio_stream::{Stream, StreamExt};

use crate::{print::Printer, util};

/// Async runner for files
pub async fn count_files<F, C, S, O>(
    mut files: F,
    counter: &C,
    stats: Stats,
    state: S,
    mut printer: Box<dyn Printer>,
) -> std::io::Result<()>
where
    F: Stream<Item = Result<String>> + Unpin,
    C: Counter<State = S, Output = O>,
    O: Collapse<Stats> + Clone,
    S: Clone + 'static,
{
    while let Some(next) = files.next().await {
        let next = next?;
        let result = get_result(next.as_str(), counter, stats.clone(), state.clone()).await;
        printer.print((next, result)).await?;
    }

    printer.close().await?;
    Ok(())
}

async fn get_result<C, S, O>(next: &str, counter: &C, stats: Stats, state: S) -> Result<Stats>
where
    C: Counter<State = S, Output = O>,
    O: Collapse<Stats> + Clone,
    S: Clone + 'static,
{
    let reader = util::path_to_bufread(next).await?;
    util::count_bufreader(reader, counter, state, stats).await
}
