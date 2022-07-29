#![cfg(any(feature = "tokio", feature = "sync"))]

use std::path::Path;

use rstest::*;
use speculoos::assert_that;

use libcw::config::{Encoding, LineBreak};
use libcw::{Parser, Stats};

#[cfg(feature = "sync")]
use std::{fs::File, io::BufReader};

#[cfg(feature = "tokio")]
use tokio::{fs::File, io::BufReader};

#[maybe_async::maybe_async]
async fn path_to_bufread(path: impl AsRef<Path>) -> BufReader<File> {
    let file = File::open(path).await.unwrap();
    BufReader::new(file)
}

#[fixture]
fn parser() -> Parser {
    Parser::new(Encoding::UTF8, LineBreak::LF, true, true, true, true, true)
}

#[maybe_async::maybe_async]
#[rstest]
#[case::empty(
    "resources/utf8/empty.txt",
    Stats::new(Some(0), Some(0), Some(0), Some(0), Some(0))
)]
#[case::french(
    "resources/utf8/french.txt",
    Stats::new(Some(0), Some(10), Some(58), Some(61), Some(58))
)]
#[case::spanish(
    "resources/utf8/spanish.txt",
    Stats::new(Some(1), Some(3), Some(19), Some(22), Some(18))
)]
#[case::small(
    "resources/utf8/small.txt",
    Stats::new(Some(1), Some(3), Some(18), Some(18), Some(17))
)]
#[case::lorem_big(
    "resources/utf8/Lorem_big.txt",
    Stats::new(Some(1996), Some(111618), Some(751539), Some(751539), Some(1142))
)]
#[trace]
#[cfg_attr(not(any(feature = "tokio")), test)]
#[cfg_attr(feature = "tokio", tokio::test)]
async fn test_file_produces_the_expected_output(
    parser: Parser,
    #[case] path: &'static str,
    #[case] expected: Stats,
) {
    let bufreader = path_to_bufread(path).await;
    let obtained = parser.process(bufreader).await.unwrap();

    assert_that!(obtained).is_equal_to(expected)
}
