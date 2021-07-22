use clap::ArgMatches;

#[derive(Debug,Default)]
pub struct Cwargs {
    lines:bool,
    words:bool,
    characters:bool,
    bytes:bool,
}

impl Cwargs {
    pub fn new(args:&ArgMatches) -> Cwargs {
        let lines = args.is_present("lines");
        let words = args.is_present("words");
        let characters = args.is_present("characters");
        let bytes = args.is_present("bytes");

        Cwargs {
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