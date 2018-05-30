use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::env;

type RunError<E> = Result<(), Box<E>>;

pub fn run(config: Config) -> RunError<Error> {
    let mut f = File::open(config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("Something went wrong with reading the file!");

    type SearchFn = for<'r, 'a> fn(&'r str, &'a str) -> Vec<&'a str>;
    let mut search_func: SearchFn = search;

    if config.case_insensitive {
        search_func = search_case_insensitive;
    }

    for line in search_func(&config.query, &contents) {
        println!("{}", line);
    }
    Ok(())
}

// avoid primitive obsession
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_insensitive: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(q) => q,
            None => return Err("No query supplied"),
        };

        let filename = match args.next() {
            Some(f) => f,
            None => return Err("No filename supplied"),
        };

        Ok(Config {
            query,
            filename,
            case_insensitive: !env::var("CASE_INSENSITIVE").is_err(),
        })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_contents() -> &'static str {
        "\
Rust
safe, fast, productive.
Pick three."
    }

    #[test]
    fn case_sensitive() {
        assert_eq!(vec!["Pick three."], search("Pick", get_contents()));
        assert_eq!(vec![] as Vec<&str>, search("pick", get_contents()));
    }

    #[test]
    fn case_insensitive() {
        assert_eq!(
            vec!["Pick three."],
            search_case_insensitive("Pick", get_contents())
        );
        assert_eq!(
            vec!["Pick three."],
            search_case_insensitive("pick", get_contents())
        );
    }

}
