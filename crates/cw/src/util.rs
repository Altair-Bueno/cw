use libcw::counter::{Collapse, Counter};
use std::path::Path;
use tokio::{
    fs::File,
    io::{AsyncBufRead, AsyncBufReadExt, BufReader},
};
use tokio_stream::{wrappers::LinesStream, Stream, StreamExt};

pub async fn count_bufreader<R, C, S, O, F>(
    mut reader: R,
    counter: C,
    mut state: S,
    collapsable: F,
) -> std::io::Result<F>
where
    R: AsyncBufRead + Unpin,
    C: Counter<State = S, Output = O>,
    S: 'static,
    F: 'static,
    O: Collapse<F>,
{
    let mut amount = 0;
    loop {
        reader.consume(amount);
        let buff = reader.fill_buf().await?;
        amount = buff.len();

        if amount == 0 {
            return Ok(counter.terminate(state).collapse(collapsable));
        } else {
            state = counter.parse(buff, state);
        }
    }
}

#[inline]
pub async fn path_to_bufread(path: impl AsRef<Path>) -> std::io::Result<impl AsyncBufRead> {
    let file = File::open(path).await?;
    Ok(BufReader::new(file))
}

#[inline]
pub async fn stdin_to_bufread() -> impl AsyncBufRead {
    BufReader::new(tokio::io::stdin())
}

#[inline]
pub async fn stdin_to_path_stream() -> impl Stream<Item = std::io::Result<String>> {
    let stdin = BufReader::new(tokio::io::stdin());
    LinesStream::new(stdin.lines())
}

