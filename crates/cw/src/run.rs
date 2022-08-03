use libcw::counter::byte::ByteCounter;
use libcw::counter::line::LineCounter;
use libcw::counter::word::WordCounter;
use tower::{layer::util::Identity, ServiceBuilder};

use crate::config::Config;

pub async fn run(_config: Config) -> u8 {
    let _counter = ServiceBuilder::new()
        .layer(ByteCounter::new())
        .layer(LineCounter::new(Default::default()))
        .layer(WordCounter::new())
        .service(Identity::new());

    //
    //     let state : ByteCounterServiceState<LineCounterServiceState<WordCounterServiceState<()>>> = Default::default();
    //
    //     let eater = AbstractEater { state, counter };
    //     let eater = Box::new(eater) as Box<dyn TapeEater<&[u8], AnyMap>>;
    //
    //     let Config {
    //         from_stdin,
    //         json,
    //         files,
    //         ..
    //     } = config;
    //
    //     let mut eater = eater;
    //     let map = eater.result(AnyMap::new());
    //
    //     eater.eat(&[1,2,3,4]);
    //
    //     print!("{:?}", map);
    0
}
