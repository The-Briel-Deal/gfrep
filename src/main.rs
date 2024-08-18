use gfrep;
use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let config = gfrep::Config::build(&args)?;

    gfrep::run(config)?;

    Ok(())
}

