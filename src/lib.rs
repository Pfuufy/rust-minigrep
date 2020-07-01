use std::error::Error;
use std::fs;
use std::env;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments. Please enter 2.");
        }

        // Immediately skip first item because it's the program
        // name which I don't care about.
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't recieve a query string")
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't receive a file name")
        };

        let case_sensitive = match args.next() {
            Some(arg) => get_bool_value(&arg),
            None => env::var("CASE_INSENSITIVE").is_err()
        };

        Ok(Config {
            query,
            filename,
            case_sensitive
        })
    }
}

pub fn get_bool_value(string: &str) -> bool {
    let string = string.to_lowercase();

    if string == "false" || string == "no" || string == "0" {
        false
    } else {
        true
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    if results.len() == 0 {
        results.push("No matches found.");
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_bool_value_false() {
        let vals =  vec!["false", "FaLsE", "no", "0"];

        for val in vals {
            assert_eq!(false, get_bool_value(val));
        }
    }

    #[test]
    fn get_bool_value_true() {
        assert_eq!(true, get_bool_value("true"));
    }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));

    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        )
    }
}
