use std::env::{self, args};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);

    read_file(&config.file_path);
}

fn read_file(path: &str) {
    use std::fs;
    let contents = fs::read_to_string(path).expect("failed to read the file");
    println!("content are:\n{contents}");
}

struct Config<'a> {
    query: &'a String,
    file_path: &'a String,
}

impl<'a> Config<'a> {
    fn new(args: &'a [String]) -> Self {
        Self {
            query: &args[1],
            file_path: &args[2],
        }
    }
}
