use std::fs::File;
use std::io::BufReader;

use cw::stats::Stats;
use cw::stats::automata::posix_utf8::PosixUTF8;
use cw::stats::automata::Automata;

fn proccess_file_test(f: &str) -> Stats {
    let reader = BufReader::new(File::open(f).unwrap());
    let stats = PosixUTF8.stats_from_bufread(Box::new(reader)).unwrap();

    stats
}

#[test]
fn gabriel() {
    let out = proccess_file_test("tests/resources/Gabriel.txt");
    let expected = Stats::new(57, 187, 2700, 2700);
    assert_eq!(out, expected)
}

#[test]
fn lorem() {
    let out = proccess_file_test("tests/resources/Lorem_big.txt");
    let expected = Stats::new(1996, 111618, 751539, 751539);
    assert_eq!(out, expected)
}
#[test]
fn bible() {
    let out = proccess_file_test("tests/resources/bible.txt");
    let expected = Stats::new(100182, 824036, 4451368, 4451368);
    assert_eq!(out, expected)
}
#[test]
fn s1() {
    let out = proccess_file_test("tests/resources/sample1.txt");
    let expected = Stats::new(3, 88, 607, 607);
    assert_eq!(out, expected)
}

#[test]
fn s2() {
    let out = proccess_file_test("tests/resources/sample2.txt");
    let expected = Stats::new(12, 423, 2859, 2859);
    assert_eq!(out, expected)
}
#[test]
fn s3() {
    let out = proccess_file_test("tests/resources/sample3.txt");
    let expected = Stats::new(20, 546, 3541, 3541);
    assert_eq!(out, expected)
}
#[test]
fn small() {
    let out = proccess_file_test("tests/resources/small.txt");
    let expected = Stats::new(1, 3, 18, 18);
    assert_eq!(out, expected)
}
#[test]
fn empty() {
    let out = proccess_file_test("tests/resources/empty.txt");
    let expected = Stats::new(0, 0, 0, 0);
    assert_eq!(out, expected)
}

#[test]
fn arabic() {
    let out = proccess_file_test("tests/resources/arabic.txt");
    let expected = Stats::new(0, 10, 58, 105);
    assert_eq!(out, expected)
}
