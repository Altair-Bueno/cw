mod json;
// mod stdout;
// pub use stdout::*;
pub use json::*;

use libcw::Stats;

pub type Message = (String, std::io::Result<Stats>);

#[async_trait::async_trait]
pub trait Printer {
    async fn print(&mut self, message: Message) -> std::io::Result<()>;
    async fn close(&mut self) -> std::io::Result<()>;
}
