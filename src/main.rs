use std::env;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    let query = args
        .get(1)
        .expect("Nothing was specified for the query param!!!");

    let filepath = args
        .get(2)
        .expect("Nothing was specified for the filepath param!!!");

    let contents = fs::read_to_string(filepath)?;

    eprintln!("With text:\n{contents}");

    Ok(())
}
