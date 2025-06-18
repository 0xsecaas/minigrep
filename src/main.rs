use minigrep::Config;
use std::{env, error::Error, process};

fn main() -> Result<(), Box<dyn Error>> {
    let config: Config = Config::build(env::args()).unwrap_or_else(|err: &'static str| {
        eprintln!("Input error: {err}");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(&config) {
        eprintln!("Application error: {e}");
        process::exit(2);
    }

    Ok(())
}
