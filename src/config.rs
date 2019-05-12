pub enum InputType {
    Stdin,
    File(String)
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
                input_type = InputType::File(file.to_string());
            }
        }

        Config {
            input_type,
            delimiter: matches.value_of("delimiter").unwrap_or("\n").chars().next().unwrap()
        }
    }
}
