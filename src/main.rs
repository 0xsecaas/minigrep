use minigrep::Config;
use std::{env, error::Error, process};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let config: Config<'_> = Config::build(&args).unwrap_or_else(|err: &'static str| {
        eprintln!("Input error: {err}");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(&config) {
        eprintln!("Application error: {e}");
        process::exit(2);
    }

    Ok(())
}
