use std::env;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let config = parse_config(&args)?;

    let contents = fs::read_to_string(config.file_path)?;

    eprintln!("With text:\n{contents}");

    Ok(())
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &str> {
        let query = args
            .get(1)
            .ok_or("Nothing was specified for the query param!!!")?
            .clone();

        let file_path = args
            .get(2)
            .ok_or("Nothing was specified for the filepath param!!!")?
            .clone();

        Ok(Config { query, file_path })
    }
}

// Returns a tuple of (query, filepath) if they both exist.
fn parse_config(args: &[String]) -> Result<Config, &str> {
    let query = args
        .get(1)
        .ok_or("Nothing was specified for the query param!!!")?
        .clone();

    let file_path = args
        .get(2)
        .ok_or("Nothing was specified for the filepath param!!!")?
        .clone();

    Ok(Config { query, file_path })
}
