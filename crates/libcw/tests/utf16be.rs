#![cfg(any(feature="tokio", feature="sync"))]
use std::path::Path;

use rstest::*;
use speculoos::assert_that;

use libcw::{Parser, Stats};
use libcw::config::{Encoding, LineBreak};

#[cfg(feature = "sync")]
use std::{fs::File, io::BufReader};

#[cfg(feature="tokio")]
use tokio::{fs::File, io::BufReader};

#[maybe_async::maybe_async]
async fn path_to_bufread(path: impl AsRef<Path>) -> BufReader<File> {
    let file = File::open(path).await.unwrap();
    BufReader::new(file)
}

#[fixture]
fn parser() -> Parser {
    Parser::new(Encoding::UTF16, LineBreak::LF, true, true, true, true, true)
}

#[maybe_async::maybe_async]
#[rstest]
#[case::empty("resources/utf16be/empty.txt", Stats::new(Some(0), Some(0), Some(0), Some(0), Some(0)))]
// #[case::french("resources/utf16be/french.txt", Stats::new(Some(0), Some(10), Some(58), Some(118), Some(58)))]
// #[case::spanish("resources/utf16be/spanish.txt", Stats::new(Some(1), Some(3), Some(19), Some(40), Some(18)))]
// #[case::small("resources/utf16be/small.txt", Stats::new(Some(1), Some(3), Some(18), Some(38), Some(17)))]
#[trace]
#[cfg_attr(not(any(feature="tokio")), test)]
#[cfg_attr(feature="tokio", tokio::test)]
async fn test_file_produces_the_expected_output(parser: Parser, #[case] path:&'static str, #[case] expected: Stats) {
    let bufreader = path_to_bufread(path).await;
    let obtained = parser.process(bufreader).await.unwrap();

    assert_that!(obtained).is_equal_to(expected)
}