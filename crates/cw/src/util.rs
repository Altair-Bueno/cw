use crate::eater::Eater;
use tokio::io::{AsyncReadExt, AsyncBufReadExt, AsyncRead, AsyncBufRead};
use tower::Layer;

pub async fn eat_async_reader<'t, 'e, 'r,  R, COLLAPSABLE>(
    eater: &'e mut dyn Eater<&'t [u8], COLLAPSABLE>,
    mut reader: R,
) -> std::io::Result<()>
where
    R: AsyncBufRead + Sized + Unpin + 'r,
    'r:'t
{
    let mut amount = 0;

    loop {
        reader.consume(amount);
        let buffer: &[u8] = reader.fill_buf().await?;
        amount = buffer.len();
        
        if amount == 0 {
            return Ok(());
        } else {

        }
    }
}
/*
pub async fn foooo() -> std::io::Result<()> {
    let counter = libcw::counter::byte::ByteCounter::new().layer(tower::layer::util::Identity::new());
    let state: libcw::counter::byte::ByteCounterServiceState<()> = Default::default();
    let mut eater = crate::eater::AbstractEater::new(state, counter);
    let reader = tokio::io::BufReader::new(tokio::fs::File::open("Cargo.lock").await?);
    eat_async_reader(&mut eater as &mut dyn Eater<_,anymap::AnyMap>, reader).await?;
    Ok(())
}
*/
