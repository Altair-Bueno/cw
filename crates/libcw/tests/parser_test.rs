mod test {
    use std::fs::File;
    use std::io::BufReader;

    use libcw::config::*;
    use libcw::Parser;
    use libcw::Stats;

    fn proccess_file_test(f: &str) -> Stats {
        let reader = BufReader::new(File::open(f).unwrap());
        let parser = Parser::new(Encoding::UTF8, LineBreak::LF, true, true, true, true, true);
        parser.proccess(reader).unwrap()
    }

    #[test]
    fn gabriel() {
        let out = proccess_file_test("resources/utf8/Gabriel.txt");
        let expected = Stats::new(Some(57), Some(187), Some(2694), Some(2700), Some(580));
        assert_eq!(out, expected)
    }

    #[test]
    fn lorem() {
        let out = proccess_file_test("resources/utf8/Lorem_big.txt");
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
    #[ignore]
    fn world() {
        let out = proccess_file_test("resources/utf8/world192.txt");
        let expected = Stats::new(
            Some(65119),
            Some(326075),
            Some(2473400),
            Some(2473400),
            Some(81),
        );
        assert_eq!(out, expected)
    }
    #[test]
    fn s1() {
        let out = proccess_file_test("resources/utf8/sample1.txt");
        let expected = Stats::new(Some(3), Some(88), Some(607), Some(607), Some(346));
        assert_eq!(out, expected)
    }

    #[test]
    fn s2() {
        let out = proccess_file_test("resources/utf8/sample2.txt");
        let expected = Stats::new(Some(12), Some(423), Some(2859), Some(2859), Some(635));
        assert_eq!(out, expected)
    }
    #[test]
    fn s3() {
        let out = proccess_file_test("resources/utf8/sample3.txt");
        let expected = Stats::new(Some(20), Some(546), Some(3541), Some(3541), Some(818));
        assert_eq!(out, expected)
    }
    #[test]
    fn small() {
        let out = proccess_file_test("resources/utf8/small.txt");
        let expected = Stats::new(Some(1), Some(3), Some(18), Some(18), Some(17));
        assert_eq!(out, expected)
    }
    #[test]
    fn empty() {
        let out = proccess_file_test("resources/utf8/empty.txt");
        let expected = Stats::new(Some(0), Some(0), Some(0), Some(0), Some(0));
        assert_eq!(out, expected)
    }

    /*
    #[test]
    #[ignore]
    fn arabic() {
        // - Legth isn't 0
        // - test weird
        let out = proccess_file_test("resources/utf8/arabic.txt");
        let expected = Stats::new(Some(0), Some(10), Some(58), Some(105), Some(0));
        assert_eq!(out, expected)
    }
     */
    #[test]
    fn spanish() {
        let out = proccess_file_test("resources/utf8/spanish.txt");
        let expected = Stats::new(Some(1), Some(3), Some(19), Some(22), Some(18));
        assert_eq!(out, expected)
    }

    #[test]
    fn french() {
        let out = proccess_file_test("resources/utf8/french.txt");
        let expected = Stats::new(Some(0), Some(10), Some(58), Some(61), Some(58));
        assert_eq!(out, expected)
    }
}
