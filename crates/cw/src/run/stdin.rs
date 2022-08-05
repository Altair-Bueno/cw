use dyn_clone::clone_box;
use libcw::{
    counter::{Collapse, Counter},
    Stats,
};

use crate::{print::Printer, statefull_counter::Eat, util};

pub async fn count_stdin(
    mut eaters: Vec<Box<dyn Eat>>,
    stats: Stats,
    mut printer: Box<dyn Printer>,
) -> std::io::Result<()> {
    let reader = util::stdin_to_bufread().await;
    let result = util::count_bufreader(reader, &mut eaters, stats).await;

    printer.print(("STDIN".to_owned(), result)).await?;

    printer.close().await
}
