mod json;
mod stdout;
use std::path::PathBuf;

pub use json::*;
pub use stdout::*;

use libcw::Stats;

pub type Message = (PathBuf, std::io::Result<Stats>);

#[async_trait::async_trait]
pub trait Printer {
    async fn print(&mut self, message: Message) -> std::io::Result<()>;
    async fn close(&mut self) -> std::io::Result<()>;
}
