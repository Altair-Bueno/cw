use crate::Parser;
use clap::ArgMatches;

pub fn parser_from_clap(args: &ArgMatches) -> Parser {
    let encoding = args
        .value_of("encoding")
        .map(|x| x.parse().unwrap_or_default())
        .unwrap_or_default();
    let breakk = args
        .value_of("break")
        .map(|x| x.parse().unwrap_or_default())
        .unwrap_or_default();
    let lines = args.is_present("lines");
    let words = args.is_present("words");
    let characters = args.is_present("characters");
    let bytes = args.is_present("bytes");
    let len = args.is_present("line_length");

    if lines || words || characters || bytes || bytes || len {
        Parser::new(encoding, breakk, lines, words, characters, bytes, len)
    } else {
        Parser::default()
    }
}
