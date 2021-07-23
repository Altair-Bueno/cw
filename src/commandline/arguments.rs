use clap::ArgMatches;

/// Convenience struct for showing the solution
#[derive(Debug, Default)]
pub struct Cwargs {
    pub lines: bool,
    pub words: bool,
    pub characters: bool,
    pub bytes: bool,
}

impl Cwargs {
    pub fn new(args: &ArgMatches) -> Cwargs {
        let lines = args.is_present("lines");
        let words = args.is_present("words");
        let characters = args.is_present("characters");
        let bytes = args.is_present("bytes");

        Cwargs {
            lines,
            words,
            characters,
            bytes,
        }
    }
}
