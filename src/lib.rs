use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

impl Config {
    pub fn from_args(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        let ignore_case = if args.len() == 4 && (args[3].eq("--ignore_case") || args[3].eq("-ign"))
        {
            true
        } else {
            env::var("IGNORE_CASE").is_ok()
        };

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line)
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result = vec![];
    let query = query.to_lowercase();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            result.push(line);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "Rust:\nFast\nSafe\nProductive\nDuct";

        assert_eq!(vec!["Productive"], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "DUCT";
        let contents = "Rust:\nFast\nSafe\nProductive";

        assert_eq!(vec!["Productive"], search_case_insensitive(query, contents));
    }
}
