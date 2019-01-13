use std::path;
use std::io::{Read, BufReader};

pub enum InputType {
    Stdin,
    File(path::PathBuf)
}

pub struct Config {
    pub input_type: InputType
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
            .get_matches();

        let mut input_type = InputType::Stdin;

        if matches.is_present("file") {
            if let Some(file) = matches.value_of("file") {
                input_type = InputType::File(path::PathBuf::from(file));
            }
        }

        Config {
            input_type
        }
    }
}
