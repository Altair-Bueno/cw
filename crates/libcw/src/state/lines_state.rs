use std::fmt::Debug;

use crate::config::LineBreak;
use crate::state::traits::{compute::Compute, partial_state::PartialState};

/// number of lines
#[derive(Copy, Clone, Debug)]
pub struct LinesState {
    line_count: usize,
    linebreak: LineBreak,
}

impl Default for LinesState {
    fn default() -> Self {
        LinesState::new(LineBreak::default())
    }
}

impl LinesState {
    pub fn new(linebreak: LineBreak) -> Self {
        LinesState {
            line_count: 0,
            linebreak,
        }
    }
}

impl PartialState for LinesState {
    type Output = usize;
    fn output(&self) -> Self::Output {
        self.line_count
    }
}

impl Compute for LinesState {
    fn utf8_compute(self, tape: &[u8]) -> Self {
        let b = self.linebreak.get_separator();
        let line_breaks = tape.iter().filter(|x| **x == b).count();

        LinesState {
            line_count: line_breaks + self.line_count,
            ..self
        }
    }
    fn utf16_compute(self, tape: &[u8]) -> Self {
        let b = self.linebreak.get_separator();
        let mut temp = true;
        let line_breaks = tape
            .iter()
            .filter(|_| {
                temp = !temp;
                temp
            })
            .filter(|x| **x == b)
            .count();
        LinesState {
            line_count: line_breaks + self.line_count,
            ..self
        }
    }
}

#[cfg(test)]
mod test {
    mod utf16 {
        use crate::config::LineBreak;
        use crate::state::lines_state::LinesState;
        use crate::state::traits::{compute::Compute, partial_state::PartialState};

        #[test]
        pub fn test1() {
            let line = "hello world"
                .encode_utf16()
                .flat_map(u16::to_be_bytes)
                .collect::<Vec<u8>>();
            let out = LinesState::new(LineBreak::LF);
            let line = line.as_slice();
            let out = out.utf16_compute(line);
            let out = out.output();
            assert_eq!(out, 0)
        }

        #[test]
        pub fn test2() {
            let line = ""
                .encode_utf16()
                .flat_map(u16::to_be_bytes)
                .collect::<Vec<u8>>();
            let out = LinesState::new(LineBreak::LF)
                .utf16_compute(line.as_slice())
                .output();
            assert_eq!(out, 0)
        }

        #[test]
        pub fn test3() {
            let line = "\n"
                .encode_utf16()
                .flat_map(u16::to_be_bytes)
                .collect::<Vec<u8>>();
            let out = LinesState::new(LineBreak::LF)
                .utf16_compute(line.as_slice())
                .output();
            assert_eq!(out, 1)
        }

        #[test]
        pub fn test4() {
            let line = "hello\n"
                .encode_utf16()
                .flat_map(u16::to_be_bytes)
                .collect::<Vec<u8>>();
            let out = LinesState::new(LineBreak::LF)
                .utf16_compute(line.as_slice())
                .output();
            assert_eq!(out, 1)
        }

        #[test]
        pub fn test5() {
            let line = "hello\nworld"
                .encode_utf16()
                .flat_map(u16::to_be_bytes)
                .collect::<Vec<u8>>();
            let out = LinesState::new(LineBreak::LF)
                .utf16_compute(line.as_slice())
                .output();
            assert_eq!(out, 1)
        }

        #[test]
        pub fn test6() {
            let line = "\nworld"
                .encode_utf16()
                .flat_map(u16::to_be_bytes)
                .collect::<Vec<u8>>();
            let out = LinesState::new(LineBreak::LF)
                .utf16_compute(line.as_slice())
                .output();
            assert_eq!(out, 1)
        }

        #[test]
        pub fn test7() {
            let line = "\nèô,sdfa"
                .encode_utf16()
                //.inspect(|x| println!("{:#x}",x))
                .flat_map(u16::to_be_bytes)
                .collect::<Vec<u8>>();
            //line.iter().for_each(|x| println!("{:#02x}",x));
            let out = LinesState::new(LineBreak::LF)
                .utf16_compute(line.as_slice())
                .output();
            assert_eq!(out, 1)
        }

        #[test]
        pub fn test8() {
            let s1 = "helloworld"
                .encode_utf16()
                .flat_map(u16::to_be_bytes)
                .collect::<Vec<u8>>();
            let s2 = "jksajksfjas a jkasjf da \n"
                .encode_utf16()
                .flat_map(u16::to_be_bytes)
                .collect::<Vec<u8>>();
            let s3 = "\nsajisffajsjdfasf"
                .encode_utf16()
                .flat_map(u16::to_be_bytes)
                .collect::<Vec<u8>>();
            let s4 = "hasisdaoasfo"
                .encode_utf16()
                .flat_map(u16::to_be_bytes)
                .collect::<Vec<u8>>();
            let out = LinesState::new(LineBreak::LF)
                .utf16_compute(s1.as_slice())
                .utf16_compute(s2.as_slice())
                .utf16_compute(s3.as_slice())
                .utf16_compute(s4.as_slice())
                .output();
            assert_eq!(out, 2)
        }
    }

    mod utf8 {
        use std::fs::File;
        use std::io::{BufReader, Read};

        use crate::config::LineBreak;
        use crate::state::lines_state::LinesState;
        use crate::state::traits::{compute::Compute, partial_state::PartialState};

        #[test]
        pub fn test1() {
            let line = "hello world".as_bytes();
            let out = LinesState::new(LineBreak::LF).utf8_compute(line).output();
            assert_eq!(out, 0)
        }

        #[test]
        pub fn test2() {
            let line = "".as_bytes();
            let out = LinesState::new(LineBreak::LF).utf8_compute(line).output();
            assert_eq!(out, 0)
        }

        #[test]
        pub fn test3() {
            let line = "\n".as_bytes();
            let out = LinesState::new(LineBreak::LF).utf8_compute(line).output();
            assert_eq!(out, 1)
        }

        #[test]
        pub fn test4() {
            let line = "hello\n".as_bytes();
            let out = LinesState::new(LineBreak::LF).utf8_compute(line).output();
            assert_eq!(out, 1)
        }

        #[test]
        pub fn test5() {
            let line = "hello\nworld".as_bytes();
            let out = LinesState::new(LineBreak::LF).utf8_compute(line).output();
            assert_eq!(out, 1)
        }

        #[test]
        pub fn test6() {
            let line = "\nworld".as_bytes();
            let out = LinesState::new(LineBreak::LF).utf8_compute(line).output();
            assert_eq!(out, 1)
        }

        #[test]
        pub fn test7() {
            let line = "\nèô,sdfa".as_bytes();
            let out = LinesState::new(LineBreak::LF).utf8_compute(line).output();
            assert_eq!(out, 1)
        }

        #[test]
        pub fn test8() {
            let out = LinesState::new(LineBreak::LF)
                .utf8_compute("helloworld".as_bytes())
                .utf8_compute("jksajksfjas a jkasjf da \n".as_bytes())
                .utf8_compute("\nsajisffajsjdfasf".as_bytes())
                .utf8_compute("hasisdaoasfo".as_bytes())
                .output();
            assert_eq!(out, 2)
        }

        // Test on files
        fn process_file_test(f: &str) -> usize {
            let mut reader = BufReader::new(File::open(f).unwrap());

            let mut state = LinesState::new(LineBreak::LF);
            let mut buff = [0; 1024];
            loop {
                let read = reader.read(&mut buff).unwrap();
                if read == 0 {
                    return state.output();
                }
                state = state.utf8_compute(&buff[0..read]);
            }
        }

        #[test]
        fn gabriel() {
            let out = process_file_test("resources/utf8/Gabriel.txt");
            let expected = 57;
            assert_eq!(out, expected)
        }

        #[test]
        fn lorem() {
            let out = process_file_test("resources/utf8/Lorem_big.txt");
            assert_eq!(out, 1996)
        }

        #[test]
        #[ignore]
        fn world192() {
            let out = process_file_test("resources/utf8/world192.txt");
            assert_eq!(out, 65119)
        }

        #[test]
        fn s1() {
            let out = process_file_test("resources/utf8/sample1.txt");
            assert_eq!(out, 3)
        }

        #[test]
        fn s2() {
            let out = process_file_test("resources/utf8/sample2.txt");
            assert_eq!(out, 12)
        }

        #[test]
        fn s3() {
            let out = process_file_test("resources/utf8/sample3.txt");
            assert_eq!(out, 20)
        }

        #[test]
        fn small() {
            let out = process_file_test("resources/utf8/small.txt");
            assert_eq!(out, 1)
        }

        #[test]
        fn empty() {
            let out = process_file_test("resources/utf8/empty.txt");
            assert_eq!(out, 0)
        }

        #[test]
        fn arabic() {
            // - Length isn't 0
            // - test weird
            let out = process_file_test("resources/utf8/arabic.txt");
            let expected = 0;
            assert_eq!(out, expected)
        }

        #[test]
        fn spanish() {
            let out = process_file_test("resources/utf8/spanish.txt");
            let expected = 1;
            assert_eq!(out, expected)
        }

        #[test]
        fn french() {
            let out = process_file_test("resources/utf8/french.txt");
            assert_eq!(out, 0)
        }
    }
}
