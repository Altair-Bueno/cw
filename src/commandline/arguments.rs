use clap::ArgMatches;

#[derive(Debug,Default)]
pub struct Arguments {
    lines:bool,
    words:bool,
    characters:bool,
    bytes:bool,
}

impl Arguments {
    pub fn get_args(args:&ArgMatches) -> Arguments {
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
    pub fn allows_lines(&self) -> bool {
        self.lines
    }
    pub fn allows_words(&self) ->bool {
        self.words
    }
    pub fn allows_characters(&self)->bool {
        self.characters
    }
    pub fn allows_bytes(&self) ->bool {
        self.bytes
    }
}