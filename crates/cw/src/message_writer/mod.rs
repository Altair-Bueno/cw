use std::io::Error;

use async_trait::async_trait;

use libcw::Stats;

pub mod default;
pub mod json;

type Message = (String, Result<Stats, Error>);

#[async_trait]
pub trait MessageWriter: Sync + Send {
    async fn message_received(&mut self, message: Message);
    async fn terminate(&mut self) -> i32;
}
