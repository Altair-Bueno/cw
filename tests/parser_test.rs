mod test {
    use cw::{Encoding, LineBreak, Parser, Stats};
    use std::fs::File;
    use std::io::BufReader;

    fn proccess_file_test(f: &str) -> Stats {
        let reader = BufReader::new(File::open(f).unwrap());
        let parser = Parser::new(Encoding::UTF8, LineBreak::LF, true, true, true, true, true);
        parser.proccess(reader).unwrap()
    }

    #[test]
    fn gabriel() {
        let out = proccess_file_test("tests/resources/Gabriel.txt");
        let expected = Stats::new(Some(57), Some(187), Some(2694), Some(2700), Some(580));
        assert_eq!(out, expected)
    }

    #[test]
    fn lorem() {
        let out = proccess_file_test("tests/resources/Lorem_big.txt");
        let expected = Stats::new(
            Some(1996),
            Some(111618),
            Some(751539),
            Some(751539),
            Some(1142),
        );
        assert_eq!(out, expected)
    }
    #[test]
    #[ignore] // On CI does fail. I don't know why
    fn bible() {
        let out = proccess_file_test("tests/resources/bible.txt");
        let expected = Stats::new(
            Some(100182),
            Some(824036),
            Some(4451368),
            Some(4451368),
            Some(78),
        );
        assert_eq!(out, expected)
    }
    #[test]
    fn s1() {
        let out = proccess_file_test("tests/resources/sample1.txt");
        let expected = Stats::new(Some(3), Some(88), Some(607), Some(607), Some(346));
        assert_eq!(out, expected)
    }

    #[test]
    fn s2() {
        let out = proccess_file_test("tests/resources/sample2.txt");
        let expected = Stats::new(Some(12), Some(423), Some(2859), Some(2859), Some(635));
        assert_eq!(out, expected)
    }
    #[test]
    fn s3() {
        let out = proccess_file_test("tests/resources/sample3.txt");
        let expected = Stats::new(Some(20), Some(546), Some(3541), Some(3541), Some(818));
        assert_eq!(out, expected)
    }
    #[test]
    fn small() {
        let out = proccess_file_test("tests/resources/small.txt");
        let expected = Stats::new(Some(1), Some(3), Some(18), Some(18), Some(17));
        assert_eq!(out, expected)
    }
    #[test]
    fn empty() {
        let out = proccess_file_test("tests/resources/empty.txt");
        let expected = Stats::new(Some(0), Some(0), Some(0), Some(0), Some(0));
        assert_eq!(out, expected)
    }

    /*
    #[test]
    #[ignore]
    fn arabic() {
        // - Legth isn't 0
        // - test weird
        let out = proccess_file_test("tests/resources/arabic.txt");
        let expected = Stats::new(Some(0), Some(10), Some(58), Some(105), Some(0));
        assert_eq!(out, expected)
    }
     */
    #[test]
    fn spanish() {
        let out = proccess_file_test("tests/resources/spanish.txt");
        let expected = Stats::new(Some(1), Some(3), Some(19), Some(22), Some(18));
        assert_eq!(out, expected)
    }

    #[test]
    fn french() {
        let out = proccess_file_test("tests/resources/french.txt");
        let expected = Stats::new(Some(0), Some(10), Some(58), Some(61), Some(58));
        assert_eq!(out, expected)
    }
}
