extern crate eachdo;

extern crate clap;


fn main() {
    clap::App::new("eachdo")
        .version("0.1.0")
        .author("toshokan <toshokan@shojigate.net>")
        .arg(clap::Arg::with_name("file")
             .short("f")
             .long("file")
             .value_name("FILE")
             .help("Read input from FILE rather than stdin")
             .takes_value(true))
        .get_matches();
    
    println!("Done!");
}
