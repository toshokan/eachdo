extern crate eachdo;
extern crate clap;

use std::io;
use std::io::{BufRead, BufReader};
use eachdo::{Config, InputType, InputStream};

fn main() {
    let config = Config::new();
    let input_stream = InputStream::new(&config.input_type);

    for line in input_stream.iter(config.delimiter) {
        println!("{}", line)
    }
    
    println!("Done!");
}
