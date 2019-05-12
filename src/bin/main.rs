use eachdo::{
    config::{InputType, Config},
    Stream, FileInput, StdinInput, InputStream,
};

fn main() -> std::io::Result<()> {
    let config = Config::new();
    let delim = config.delimiter;
    match config.input_type {
        InputType::File(path) => {
            let stream = Stream::from(
                FileInput::new(path, delim)?
            );
            handle_input(stream);
        }
        _ => {
            let stream = Stream::from(
                StdinInput::new(delim)
            );
            handle_input(stream);
        }
    };
    Ok(())
}

fn handle_input<T: InputStream>(mut stream: Stream<T>) {
    while let Some(line) = stream.next() {
        print!("{}", line);
    }
}
