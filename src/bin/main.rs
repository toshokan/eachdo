extern crate eachdo;
extern crate clap;


fn main() {
    let config = eachdo::Config::new();
    
    if let Some(file) = config.filename {
        println!("Got a filename! {:?}", file);
    }
    
    println!("Done!");
}
