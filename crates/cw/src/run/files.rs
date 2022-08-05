use libcw::Stats;
use std::{
    io::Result,
    path::{Path, PathBuf},
};

use tokio_stream::{Stream, StreamExt};

use crate::{print::Printer, statefull_counter::Eat, util};

/// Async runner for files
pub async fn count_files<F>(
    mut files: F,
    eaters: Vec<Box<dyn Eat + Send>>,
    stats: Stats,
    mut printer: Box<dyn Printer + Send>,
) -> eyre::Result<()>
where
    F: Stream<Item = Result<PathBuf>> + Unpin,
{
    let (sender, mut receiver) = tokio::sync::mpsc::channel(20);


    let consumer_handle = tokio::spawn(async move {
        while let Some(next) = receiver.recv().await {
            printer.print(next).await?;
        }
        printer.close().await?;
        Ok::<_,std::io::Error>(())
    });

    
    while let Some(next) = files.next().await {
        let next = next?;
        let sender = sender.clone();
        let eaters: Vec<_> = eaters.iter().map(|x| dyn_clone::clone_box(&**x)).collect();

        tokio::spawn(async move {
            let result = get_result(next.as_path(), eaters, stats).await;
            sender.send((next, result)).await
        });
    }
    // Sender is no longer needed. Drop before await
    std::mem::drop(sender);
    
    consumer_handle.await??;
    Ok(())
}

async fn get_result(
    next: impl AsRef<Path>,
    eaters: Vec<Box<dyn Eat + Send>>,
    stats: Stats,
) -> Result<Stats> {
    let reader = util::path_to_bufread(next).await?;
    util::count_bufreader(reader, eaters, stats).await
}
