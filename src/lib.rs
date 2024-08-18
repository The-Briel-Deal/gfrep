use std::fs;

pub struct Config {
    query: String,
    file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
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

pub fn run(config: Config) -> Result<(), std::io::Error> {
    let contents = fs::read_to_string(config.file_path)?;
    search(&config.query, &contents)
        .iter()
        .for_each(|l| println!("{l}"));
    Ok(())
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
                        Rust:\n\
                        safe, fast, productive.\n\
                        Pick three.\n\
                       ";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn three_results() {
        let query = "o";
        let contents = "\
                        Dogs:\n\
                        big, girthy, monsters.\n\
                        Oh, and don't forget greasy.\n\
                        Pick 4.\n\
                       ";
        assert_eq!(
            vec![
                "Dogs:",
                "big, girthy, monsters.",
                "Oh, and don't forget greasy."
            ],
            search(query, contents)
        );
    }
    #[test]
    fn numerical_one_result() {
        let query = "4";
        let contents = "\
                        Dogs:\n\
                        big, girthy, monsters.\n\
                        Oh, and don't forget greasy.\n\
                        Pick 4.\n\
                       ";
        assert_eq!(vec!["Pick 4."], search(query, contents));
    }

    #[test]
    fn wrong_case_no_result() {
        let query = "pick";
        let contents = "\
                        Dogs:\n\
                        big, girthy, monsters.\n\
                        Oh, and don't forget greasy.\n\
                        Pick 4.\n\
                       ";
        assert_eq!(vec![] as Vec<&str>, search(query, contents));
    }
}
