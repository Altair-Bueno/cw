#[cfg(feature = "tokio")]
mod tokio {
    use std::path::Path;

    use rstest::*;
    use speculoos::assert_that;
    use tokio::fs::File;
    use tokio::io::BufReader;

    use libcw::{Parser, Stats};
    use libcw::config::{Encoding, LineBreak};

    #[fixture]
    fn parser() -> Parser {
        Parser::new(Encoding::UTF8, LineBreak::LF, true, true, true, true, true)
    }

    async fn path_to_bufread(path: impl AsRef<Path>) -> BufReader<File> {
        let file = File::open(path).await.unwrap();
        BufReader::new(file)
    }

    #[rstest]
    #[case::empty(path_to_bufread("resources/utf8/empty.txt").await, Stats::new(Some(0), Some(0), Some(0), Some(0), Some(0)))]
    #[case::french(path_to_bufread("resources/utf8/french.txt").await, Stats::new(Some(0), Some(10), Some(58), Some(61), Some(58)))]
    #[case::spanish(path_to_bufread("resources/utf8/spanish.txt").await, Stats::new(Some(1), Some(3), Some(19), Some(22), Some(18)))]
    #[case::small(path_to_bufread("resources/utf8/small.txt").await, Stats::new(Some(1), Some(3), Some(18), Some(18), Some(17)))]
    #[case::lorem_big(path_to_bufread("resources/utf8/Lorem_big.txt").await, Stats::new(Some(1996), Some(111618), Some(751539), Some(751539), Some(1142)))]
    #[trace]
    #[tokio::test]
    async fn test_file_produces_the_expected_output(parser: Parser, #[case] bufreader: BufReader<File>, #[case] expected: Stats) {
        let obtained = parser.process(bufreader).await.unwrap();

        assert_that!(obtained).is_equal_to(expected)
    }
}

#[cfg(not(feature = "tokio"))]
mod blocking {
    use std::fs::File;
    use std::io::BufReader;
    use std::path::Path;

    use rstest::*;
    use speculoos::assert_that;

    use libcw::{Parser, Stats};
    use libcw::config::{Encoding, LineBreak};

    #[fixture]
    fn parser() -> Parser {
        Parser::new(Encoding::UTF8, LineBreak::LF, true, true, true, true, true)
    }

    fn path_to_bufread(path: impl AsRef<Path>) -> BufReader<File> {
        let file = File::open(path).unwrap();
        BufReader::new(file)
    }

    #[rstest]
    #[case::empty(path_to_bufread("resources/utf8/empty.txt"), Stats::new(Some(0), Some(0), Some(0), Some(0), Some(0)))]
    #[case::french(path_to_bufread("resources/utf8/french.txt"), Stats::new(Some(0), Some(10), Some(58), Some(61), Some(58)))]
    #[case::spanish(path_to_bufread("resources/utf8/spanish.txt"), Stats::new(Some(1), Some(3), Some(19), Some(22), Some(18)))]
    #[case::small(path_to_bufread("resources/utf8/small.txt"), Stats::new(Some(1), Some(3), Some(18), Some(18), Some(17)))]
    #[case::lorem_big(path_to_bufread("resources/utf8/Lorem_big.txt"), Stats::new(Some(1996), Some(111618), Some(751539), Some(751539), Some(1142)))]
    #[trace]
    fn test_file_produces_the_expected_output(parser: Parser, #[case] bufreader: BufReader<File>, #[case] expected: Stats) {
        let obtained = parser.process(bufreader).unwrap();

        assert_that!(obtained).is_equal_to(expected)
    }
}