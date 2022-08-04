pub mod config;
pub mod counter;
#[cfg(feature="stats")]
mod stats;
#[cfg(feature="stats")]
pub use stats::*;


/*
use anymap::AnyMap;
use counter::{byte::ByteCounter, Collapse, Counter, line::LineCounter};
use tower_layer::Identity;
// TODO remove eventualy this thing is sick!!!!
fn __foo() {
    let service = tower::ServiceBuilder::new()
    .layer(ByteCounter::new())
    .layer(LineCounter::new(config::LineBreak::LF))
    .service(Identity::new());

    let result = service.parse(&[10, 20], Default::default());
    let _foo = service.terminate(result).collapse(AnyMap::new());
}

*/
