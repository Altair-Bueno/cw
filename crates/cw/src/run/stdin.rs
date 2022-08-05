use libcw::Stats;

use crate::{print::Printer, statefull_counter::Eat, util};

pub async fn count_stdin(
    mut eaters: Vec<Box<dyn Eat>>,
    stats: Stats,
    mut printer: Box<dyn Printer>,
) -> std::io::Result<()> {
    let reader = util::stdin_to_bufread().await;
    let result = util::count_bufreader(reader, &mut eaters, stats).await;

    printer.print(("".into(), result)).await?;

    printer.close().await
}
