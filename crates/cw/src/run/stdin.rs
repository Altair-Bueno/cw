use libcw::Stats;

use crate::{print::Printer, statefull_counter::Eat, util};

pub async fn count_stdin(
    eaters: Vec<Box<dyn Eat + Send>>,
    stats: Stats,
    mut printer: Box<dyn Printer>,
) -> std::io::Result<()> {
    let reader = util::stdin_to_bufread().await;
    let result = util::count_bufreader(reader, eaters, stats).await;

    printer.print(("".into(), result)).await?;

    printer.close().await
}
