use std::path;

pub struct Config {
    pub filename: Option<path::PathBuf>,
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

        let mut filename = None;

        if matches.is_present("file") {
            if let Some(provided_filename) = matches.value_of("file") {
                filename = Some(path::PathBuf::from(provided_filename));
            }
        }

        Config {
            filename
        }
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}


