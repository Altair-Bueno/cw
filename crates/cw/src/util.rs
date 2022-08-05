use libcw::{
    counter::{Collapse, Counter},
    Stats,
};
use std::path::Path;
use tokio::{
    fs::File,
    io::{AsyncBufRead, AsyncBufReadExt, BufReader},
};
use tokio_stream::{wrappers::LinesStream, Stream};

use crate::statefull_counter::Eat;

pub async fn count_bufreader<R>(
    mut reader: R,
    eaters: &mut [Box<dyn Eat>],
    collapsable: Stats,
) -> std::io::Result<Stats>
where
    R: AsyncBufRead + Unpin,
{
    let mut amount = 0;
    loop {
        reader.consume(amount);
        let buff = reader.fill_buf().await?;
        amount = buff.len();

        if amount == 0 {
            let obtained = eaters
                .iter_mut()
                .fold(collapsable, |acc, next| next.terminate(acc));
            return Ok(obtained);
        } else {
            for eater in eaters.iter_mut() {
                eater.eat(buff)
            }
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
