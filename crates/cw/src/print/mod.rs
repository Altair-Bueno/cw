mod json;
pub use json::*;

use crate::stats::Stats;

pub type Message = (String, std::io::Result<Stats>);

#[async_trait::async_trait]
pub trait Printer {
    async fn begin(&mut self);
    async fn print(&mut self, message: Message);
    async fn terminate(&mut self);
}

