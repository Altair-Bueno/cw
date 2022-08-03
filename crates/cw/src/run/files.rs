use anymap::AnyMap;
use libcw::counter::{Collapse, Counter};
use tokio_stream::Stream;

use crate::print::Printer;

pub async fn count_files<F, C, S, O>(
    _files: F,
    _counter: C,
    _state: S,
    _printer: &mut Box<dyn Printer + Send + Sync>,
) -> std::io::Result<()>
where
    C: Counter<State = S, Output = O>,
    S: 'static + Clone,
    O: Collapse<AnyMap>,
    F: Stream<Item = String> + Unpin,
{
    /*
    let mut stream = stdin(counter, state)
    .await
    .map(|x| x.map(TryFrom::try_from).map(Result::unwrap))
    .map(|x| ("STDIN".to_owned(), x));
    while let Some(message) = stream.next().await {
        printer.print(message).await;
    }
    */
    Ok(())
}