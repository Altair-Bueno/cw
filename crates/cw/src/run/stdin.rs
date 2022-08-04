use libcw::{
    counter::{Collapse, Counter},
    Stats,
};

use crate::{print::Printer, util};

pub async fn count_stdin<C, S, O>(
    counter: C,
    stats: Stats,
    state: S,
    mut printer: Box<dyn Printer>,
) -> std::io::Result<()>
where
    C: Counter<State = S, Output = O>,
    S: 'static,
    O: Collapse<Stats>,
{
    let reader = util::stdin_to_bufread().await;
    let result = util::count_bufreader(reader, counter, state, stats).await;

    printer.print(("STDIN".to_owned(), result)).await?;

    printer.close().await
}
