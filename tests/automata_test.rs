use cw::stats::{Stats, Automata};
use std::io::{BufReader, Read};
use std::fs::File;

fn proccess_file(f: &str) -> Stats {
    let reader = BufReader::new(File::open(f).unwrap());
    let stats = Stats::from_file(Box::new(reader)).unwrap();

    stats
}

#[test]
fn awonderfull () {
   let out = proccess_file("tests/resources/A wonderful serenity has taken \
   possessio.rtf");
    let expected = Stats::new(12,219,1536,1536);
    assert_eq!(out,expected)
}
#[test]
fn gabriel () {
    let out = proccess_file("tests/resources/Gabriel.txt");
    let expected = Stats::new(57,187,2700,2700);
    assert_eq!(out,expected)
}
#[test]
fn li_europan () {
    let out = proccess_file("tests/resources/Li Europan lingues es membres del sam fa.rtf");
    let expected = Stats::new(12,218,1706,1706);
    assert_eq!(out,expected)
}

#[test]
fn lorem () {
    let out = proccess_file("tests/resources/Lorem ipsum dolor sit amet, consectetur .rtf");
    let expected = Stats::new(13,478,3446,3446);
    assert_eq!(out,expected)
}

#[test]
fn one_morning () {
    let out = proccess_file("tests/resources/One morning, when Gregor Samsa woke from.rtf");
    let expected = Stats::new(12,219,1540,1540);
    assert_eq!(out,expected)
}

#[test]
fn sed_ut () {
    let out = proccess_file("tests/resources/Sed ut perspiciatis unde omnis iste natu.rtf");
    let expected = Stats::new(13,219,1797,1797);
    assert_eq!(out,expected)
}

/*
      12     219    1536 A wonderful serenity has taken possessio.rtf
      57     187    2700 Gabriel.txt
      12     218    1706 Li Europan lingues es membres del sam fa.rtf
      13     478    3446 Lorem ipsum dolor sit amet, consectetur .rtf
      12     219    1540 One morning, when Gregor Samsa woke from.rtf
      12     219    1797 Sed ut perspiciatis unde omnis iste natu.rtf
      12     218    1506 abc def ghi jkl mno pqrs tuv wxyz ABC DE copia.rtf
      13     236    1510 abc def ghi jkl mno pqrs tuv wxyz ABC DE.rtf
  100182  824036 4451368 bible.txt
 */