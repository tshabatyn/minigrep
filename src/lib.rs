// Copyright 2024 Taras Shabatyn
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::error::Error;
use std::fs;

const CASE_INSENSETIVE_OPTION: &str = "-i";

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub filename: String,
    pub is_case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        let is_case_sensitive = if args.iter().filter(|arg| arg == &&String::from(CASE_INSENSETIVE_OPTION)).count() > 0 {
            false
        } else {
            true
        };

        let args = args.iter().filter(|arg| arg != &&String::from(CASE_INSENSETIVE_OPTION)).map(|arg| arg.to_string()).collect::<Vec<String>>();

        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename, is_case_sensitive})
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let line = if config.is_case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };
    println!("{}", line.join("\n"));

    Ok(())
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines()
        .filter(|line| line.contains(query))
        .collect()
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents.lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }

    #[test]
    fn case_sensitive() {
        let query = "Rust";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:"],
            search(query, contents)
        );
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

    #[test]
    fn case_insensitive_option() {
        let args = vec!["minigrep".to_string(), "-i".to_string(), "rUsT".to_string(), "test.txt".to_string()];
        let config = Config::new(&args).unwrap();

        assert_eq!(config.is_case_sensitive, false);
    }

    #[test]
    fn case_sensitive_option() {
        let args = vec!["minigrep".to_string(), "rUsT".to_string(), "test.txt".to_string()];
        let config = Config::new(&args).unwrap();

        assert_eq!(config.is_case_sensitive, true);
    }

    #[test]
    #[should_panic]
    fn not_enough_arguments() {
        let args = vec!["minigrep".to_string(), "rUsT".to_string()];
        let _ = Config::new(&args).unwrap();
    }
}
