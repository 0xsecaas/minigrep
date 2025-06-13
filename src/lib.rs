use std::error::Error;

pub struct Config<'a> {
    pub query: &'a String,
    pub file_path: &'a String,
}

impl<'a> Config<'a> {
    pub fn build(args: &'a [String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        Ok(Self {
            query: &args[1],
            file_path: &args[2],
        })
    }
}

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let file_contents = read_file(config.file_path)?;
    println!("content are:\n{file_contents}");
    Ok(())
}

fn read_file(path: &str) -> Result<String, Box<dyn std::error::Error>> {
    use std::fs;
    let contents = fs::read_to_string(path)?;
    Ok(contents)
}
