use std::path;
use std::io;
use std::io::{BufRead, BufReader};

pub enum InputType {
    Stdin,
    File(path::PathBuf)
}

pub struct Config {
    pub input_type: InputType,
    pub delimiter: char,
}

impl Config {
    pub fn new() -> Config {
        let matches = clap::App::new("eachdo")
            .version("0.1.0")
            .author("toshokan <toshokan@shojigate.net>")
            .arg(clap::Arg::with_name("file")
                 .short("f")
                 .long("file")
                 .value_name("FILE")
                 .help("Read input from FILE rather than stdin")
                 .takes_value(true))
            .arg(clap::Arg::with_name("delimiter")
                 .short("d")
                 .long("delimiter")
                 .value_name("BYTE")
                 .takes_value(true)
                 .default_value("\n")
                 .help("Split input by BYTE"))
            .get_matches();

        let mut input_type = InputType::Stdin;

        if matches.is_present("file") {
            if let Some(file) = matches.value_of("file") {
                input_type = InputType::File(path::PathBuf::from(file));
            }
        }

        Config {
            input_type,
            delimiter: matches.value_of("delimiter").unwrap_or("\n").chars().next().unwrap()
        }
    }
}

pub struct InputStream {
    pub stream: Box<dyn BufRead>,
}

impl InputStream {
    pub fn new(input_type: &InputType) -> InputStream {
        let stream: Box<dyn BufRead> = match input_type {
            InputType::File(filename) => {
                let f = std::fs::File::open(filename)
                    .expect("Error opening file");

                Box::from(BufReader::new(f))
            },
            InputType::Stdin => Box::from(BufReader::new(io::stdin())),
        };

        InputStream {
            stream
        }
    }

    pub fn iter(self, delim: char) -> Box<dyn Iterator<Item=String>> {
        Box::from(self.stream.split(delim as u8).map(move |line| String::from_utf8(line.unwrap()).unwrap()).into_iter())
    }
}
