use libcw::Stats;
use std::{io::Result, path::{PathBuf, Path}};

use tokio_stream::{Stream, StreamExt};

use crate::{print::Printer, statefull_counter::Eat, util};

/// Async runner for files
pub async fn count_files<F>(
    mut files: F,
    eaters: Vec<Box<dyn Eat>>,
    stats: Stats,
    mut printer: Box<dyn Printer>,
) -> std::io::Result<()>
where
    F: Stream<Item = Result<PathBuf>> + Unpin,
{
    while let Some(next) = files.next().await {
        let mut eaters: Vec<_> = eaters.iter().map(|x| dyn_clone::clone_box(&**x)).collect();
        let next = next?;
        let result = get_result(next.as_path(), &mut eaters, stats).await;
        printer.print((next, result)).await?;
    }

    printer.close().await?;
    Ok(())
}

async fn get_result(next: impl AsRef<Path>, eaters: &mut [Box<dyn Eat>], stats: Stats) -> Result<Stats> {
    let reader = util::path_to_bufread(next).await?;
    util::count_bufreader(reader, eaters, stats).await
}
