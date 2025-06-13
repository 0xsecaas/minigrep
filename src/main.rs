use std::{env, error::Error, process};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Input error: {err}");
        process::exit(1);
    });

    if let Err(e) = run(&config) {
        println!("Application error: {e}");
        process::exit(2);
    }

    Ok(())
}

struct Config<'a> {
    query: &'a String,
    file_path: &'a String,
}

impl<'a> Config<'a> {
    fn build(args: &'a [String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        Ok(Self {
            query: &args[1],
            file_path: &args[2],
        })
    }
}

fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let file_contents = read_file(config.file_path)?;
    println!("content are:\n{file_contents}");
    Ok(())
}

fn read_file(path: &str) -> Result<String, Box<dyn std::error::Error>> {
    use std::fs;
    let contents = fs::read_to_string(path)?;
    Ok(contents)
}
