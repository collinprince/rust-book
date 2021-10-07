use std::{env, error::Error, fs};

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
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
    let query: String = query.to_lowercase();

    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn empty_config_arg() {
        let mut ret = false;
        if let Err(_) = Config::new(&[]) {
            ret = true;
        }
        assert!(
            ret,
            "Config::new did not throw an Err for empty vector param"
        );
    }

    #[test]
    pub fn single_config_arg() {
        let mut ret = false;
        if let Err(_) = Config::new(&[String::from("one")]) {
            ret = true;
        }
        assert!(
            ret,
            "Config::new did not throw an Err for a vector with one element"
        );
    }

    #[test]
    pub fn correct_args() {
        let mut ret = false;
        if let Ok(_) = Config::new(&[
            String::from("bin"),
            String::from("one"),
            String::from("two"),
        ]) {
            ret = true;
        }

        assert!(
            ret,
            "Config::new did not return an Ok for a vector with two elements"
        );
    }

    #[test]
    pub fn file_does_not_exist() {
        let mut ret = false;
        if let Err(_) = run(Config {
            query: String::from("query"),
            filename: String::from("gobblydeegook"),
            case_sensitive: false,
        }) {
            ret = true;
        }
        assert!(
            ret,
            "run does not return false when Config.filename does not correspond to existing file"
        );
    }

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
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
        );
    }
}
