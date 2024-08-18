use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let config = gfrep::Config::build(env::args())?;

    gfrep::run(config)?;

    Ok(())
}

