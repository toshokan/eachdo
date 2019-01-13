extern crate eachdo;
extern crate clap;

use std::io;
use std::io::{BufRead, BufReader};
use eachdo::{Config, InputType};

fn main() {
    let config = Config::new();
    let stream: Box<dyn BufRead> = match config.input_type {
        InputType::File(filename) => {
            let f = std::fs::File::open(filename)
                .expect("Error opening file");
            
            Box::from(BufReader::new(f))
        },
        InputType::Stdin => Box::from(BufReader::new(io::stdin()))
    };
    
    for line in stream.lines() {
        println!("{}", line.unwrap());
    }
    
    println!("Done!");
}
