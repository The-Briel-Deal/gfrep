use std::env;
use std::fs;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    let query = args.get(1);
    if let None = query {
        println!("Nothing was specified for the query param!!!");
        panic!();
    }
    let query = query.unwrap();

    let filepath = args.get(2);
    if let None = filepath {
        println!("Nothing was specified for the filepath param!!!");
        panic!();
    }
    let filepath = filepath.unwrap();

    println!(
        "Thank you for frepping with me, the query is '{query}', \
        and the filepath is '{filepath}'"
    );

    let contents = fs::read_to_string(filepath)?;

    println!("With text:\n{contents}");

    Ok(())
}
