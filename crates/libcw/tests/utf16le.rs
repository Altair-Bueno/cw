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
        Parser::new(Encoding::UTF16, LineBreak::LF, true, true, true, true, true)
    }

    async fn path_to_bufread(path: impl AsRef<Path>) -> BufReader<File> {
        let file = File::open(path).await.unwrap();
        BufReader::new(file)
    }

    #[rstest]
    #[case::empty(path_to_bufread("resources/utf16le/empty.txt").await, Stats::new(Some(0), Some(0), Some(0), Some(2), Some(0)))]
    #[case::french(path_to_bufread("resources/utf16le/french.txt").await, Stats::new(Some(0), Some(10), Some(58), Some(118), Some(58)))]
    #[case::spanish(path_to_bufread("resources/utf16le/spanish.txt").await, Stats::new(Some(1), Some(3), Some(19), Some(40), Some(18)))]
    #[case::small(path_to_bufread("resources/utf16le/small.txt").await, Stats::new(Some(1), Some(3), Some(18), Some(38), Some(17)))]
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
        Parser::new(Encoding::UTF16, LineBreak::LF, true, true, true, true, true)
    }

    fn path_to_bufread(path: impl AsRef<Path>) -> BufReader<File> {
        let file = File::open(path).unwrap();
        BufReader::new(file)
    }

    #[rstest]
    #[case::empty(path_to_bufread("resources/utf16le/empty.txt"), Stats::new(Some(0), Some(0), Some(0), Some(2), Some(0)))]
    #[case::french(path_to_bufread("resources/utf16le/french.txt"), Stats::new(Some(0), Some(10), Some(58), Some(118), Some(58)))]
    #[case::spanish(path_to_bufread("resources/utf16le/spanish.txt"), Stats::new(Some(1), Some(3), Some(19), Some(40), Some(18)))]
    #[case::small(path_to_bufread("resources/utf16le/small.txt"), Stats::new(Some(1), Some(3), Some(18), Some(38), Some(17)))]
    #[trace]
    fn test_file_produces_the_expected_output(parser: Parser, #[case] bufreader: BufReader<File>, #[case] expected: Stats) {
        let obtained = parser.process(bufreader).unwrap();

        assert_that!(obtained).is_equal_to(expected)
    }
}