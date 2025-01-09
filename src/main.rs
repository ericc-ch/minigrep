use std::{
    env::{self, Args},
    fs,
};

fn main() {
    let args = parse_args(env::args());
    let config = Config::new(&args);

    let file_content = fs::read_to_string(config.file_path).expect("File cannot be read!");

    println!("file content: {file_content}");
}

fn parse_args(args: Args) -> Vec<String> {
    args.skip(1).collect()
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        let query = args.first().expect("No query provided!").to_owned();

        let file_path = args.get(1).expect("No file path provided!").to_string();

        Config { query, file_path }
    }
}
