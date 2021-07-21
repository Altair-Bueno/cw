use clap::ArgMatches;

#[derive(Debug,Default)]
pub struct Arguments {
    lines:bool,
    words:bool,
    characters:bool,
    bytes:bool,
}

impl Arguments {
    pub fn get_args(args:ArgMatches) -> Arguments {
        // TODO complete
        let lines = args.is_present("lines");
        let words = false;
        let characters = false;
        let bytes = false;

        Arguments {
            lines,
            words,
            characters,
            bytes
        }
    }
}