use anymap::AnyMap;
use libcw::counter::{Collapse, Counter};
use tokio_stream::{Stream, StreamExt};

use crate::{print::Printer, util};

pub async fn count_stdin<C, S, O>(
    counter: C,
    state: S,
    printer: &mut dyn Printer,
) -> std::io::Result<()>
where
    C: Counter<State = S, Output = O>,
    S: 'static,
    O: Collapse<AnyMap>,
{
    let mut stream = stdin(counter, state)
        .await
        .map(|x| x.map(TryFrom::try_from).map(Result::unwrap))
        .map(|x| ("STDIN".to_owned(), x));
    while let Some(message) = stream.next().await {
        printer.print(message).await;
    }
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
