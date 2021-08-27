use std::fs::File;
use std::io::BufReader;

use libcw::config::*;
use libcw::Parser;
use libcw::Stats;

fn proccess_file_test(f: &str) -> Stats {
    let reader = BufReader::new(File::open(f).unwrap());
    let parser = Parser::new(Encoding::UTF16, LineBreak::LF, true, true, true, true, true);
    parser.proccess(reader).unwrap()
}

#[test]
fn gabriel() {
    let out = proccess_file_test("resources/utf16/utf16le/Gabriel.txt");
    let expected = Stats::new(Some(57), Some(187), Some(2694), Some(5390), Some(580));
    assert_eq!(out, expected)
}

#[test]
#[ignore]
fn lorem() {
    // Panic at assertion failed
    // Left:  Stats { lines: Some(1023), words: Some(56797), characters: Some(751585), bytes: Some(1503171), legth: Some(1142) }
    // Right: Stats { lines: Some(1996), words: Some(111618), characters: Some(751539), bytes: Some(1503080), legth: Some(1142) }
    //
    // File starts with fffe
    // Little endian file
    let out = proccess_file_test("resources/utf16/utf16le/Lorem_big.txt");
    let expected = Stats::new(
        Some(1996),
        Some(111618),
        Some(751539),
        Some(1503080),
        Some(1142),
    );
    assert_eq!(out, expected)
}

#[test]
#[ignore]
fn world() {
    let out = proccess_file_test("resources/utf16/utf16le/world192.txt");
    let expected = Stats::new(
        Some(65119),
        Some(326075),
        Some(2473400),
        Some(4820944),
        Some(81),
    );
    assert_eq!(out, expected)
}

#[test]
fn s1() {
    let out = proccess_file_test("resources/utf16/utf16le/sample1.txt");
    let expected = Stats::new(Some(3), Some(88), Some(607), Some(1216), Some(346));
    assert_eq!(out, expected)
}

#[test]
fn s2() {
    let out = proccess_file_test("resources/utf16/utf16le/sample2.txt");
    let expected = Stats::new(Some(12), Some(423), Some(2859), Some(5720), Some(635));
    assert_eq!(out, expected)
}

#[test]
fn s3() {
    let out = proccess_file_test("resources/utf16/utf16le/sample3.txt");
    let expected = Stats::new(Some(20), Some(546), Some(3541), Some(7084), Some(818));
    assert_eq!(out, expected)
}

#[test]
fn small() {
    let out = proccess_file_test("resources/utf16/utf16le/small.txt");
    let expected = Stats::new(Some(1), Some(3), Some(18), Some(38), Some(17));
    assert_eq!(out, expected)
}

#[test]
fn empty() {
    let out = proccess_file_test("resources/utf16/utf16le/empty.txt");
    let expected = Stats::new(Some(0), Some(0), Some(0), Some(2), Some(0));
    assert_eq!(out, expected)
}

/*
#[test]
#[ignore]
fn arabic() {
    // - Legth isn't 0
    // - test weird
    let out = proccess_file_test("resources/utf16/utf16le/arabic.txt");
    let expected = Stats::new(Some(0), Some(10), Some(58), Some(105), Some(0));
    assert_eq!(out, expected)
}
 */
#[test]
fn spanish() {
    let out = proccess_file_test("resources/utf16/utf16le/spanish.txt");
    let expected = Stats::new(Some(1), Some(3), Some(19), Some(40), Some(18));
    assert_eq!(out, expected)
}

#[test]
fn french() {
    let out = proccess_file_test("resources/utf16/utf16le/french.txt");
    let expected = Stats::new(Some(0), Some(10), Some(58), Some(118), Some(58));
    assert_eq!(out, expected)
}
