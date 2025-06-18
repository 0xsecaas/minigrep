use std::{env, error::Error};

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(
        // args: &'a [String],
        mut args: impl Iterator<Item = String>,
    ) -> Result<Self, &'static str> {
        args.next(); // drop the first argument.

        let query: String = match args.next() {
            Some(i) => i,
            None => return Err("Query not found!"),
        };

        let file_path = match args.next() {
            Some(args) => args,
            None => return Err("File path not found!"),
        };

        let case_insensitive = match args.next() {
            Some(_) => true,
            _ => false,
        };

        Ok(Self {
            query: query,
            file_path: file_path,
            ignore_case: env::var("IGNORE_CASE").is_ok() || case_insensitive,
            // ignore_case: env::var("IGNORE_CASE").is_ok() || args.contains(&"-i".to_string()),
        })
    }
}

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let file_contents = read_file(config.file_path.clone())?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &file_contents)
    } else {
        search(&config.query, &file_contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

fn read_file(path: String) -> Result<String, Box<dyn std::error::Error>> {
    use std::fs;
    let contents = fs::read_to_string(path)?;
    Ok(contents)
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results: Vec<&str> = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests {

    use super::*;

    const CONTENTS: &'static str = "\
Rust:
safe, fast, productive.
Pick three.";

    #[test]
    fn one_result() {
        let query = "duct";
        assert_eq!(vec!["safe, fast, productive."], search(query, CONTENTS));
    }

    #[test]
    fn case_sensitive() {
        let query = "rUsT";

        assert_eq!(search_case_insensitive(query, CONTENTS), vec!["Rust:"]);
    }
}
