use clap::ArgMatches;
use crate::Parser;

pub fn parser_from_clap(args:&ArgMatches) -> Parser {
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

    if lines == words && lines == characters && lines == bytes && lines == len {
        Parser::default()
    } else {
        Parser::new(encoding,breakk,lines,words,characters,bytes,len)
    }
}