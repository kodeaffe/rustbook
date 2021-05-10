//! # Minigrep
//!
//! `minigrep` is a tool to grep for strings in a file
//!
//! ## Examples
//!
//! Just call the run function to see it in action:
//! ```
//! minigrep::run()
//! ```
//!
use std::env;
use std::error::Error;
use std::fs;
use std::process;

///Configuration for grepping
pub struct Config {
    /// Query string to search for
    query: String,
    /// Name of the file to search in
    filename: String,
    /// If the search will be case sensitive
    case_sensitive: bool,
}

impl Config {
    fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next(); // skip program name
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("No query string!"),
        };
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("No filename!"),
        };
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config { query, filename, case_sensitive })
    }
}

/// Greps for a query string as given in config
///
/// # Panics
///
/// No panic intended.
///
///
/// # Errors
///
/// Returns with an Error if file cannot be read (anything std::fs::read_to_string chokes on).
///
///
/// # Safety
///
/// This should be memory safe.
///
///
/// # Examples
///
/// ```
/// if let Err(err) = grep(config) {
///     eprintln!("Something went wrong reading the file: {:?}", err);
///     process::exit(2);
///  };
/// panic!("fdasf");
/// ```
fn grep(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    //println!("With text:\n{}", contents);
    for result in search(config.query, contents, config.case_sensitive) {
        println!("{}: {}", result.line_no, result.line);
    }
    Ok(())
}


#[derive(Debug, PartialEq)]
struct SearchResult {
    line_no: usize,
    line: String,
}

fn search(mut query: String, contents: String, case_sensitive: bool) -> Vec<SearchResult> {
    let mut results = Vec::new();
    let mut line_no = 1;
    if !case_sensitive {
        query = query.to_lowercase();
    }
    for line in contents.lines() {
        let mut push = false;
        if case_sensitive && line.contains(&query) {
           push = true;
        } else if !case_sensitive && line.to_lowercase().contains(&query) {
           push = true;
        }
        if push {
            results.push(SearchResult { line_no: line_no, line: line.to_string() });
        }
        line_no += 1;
    }
    results
}

pub fn run() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing args: {}", err);
        process::exit(1);
    });
    //println!("Searching for {}", config.query);
    //println!("In file {}", config.filename);

    if let Err(err) = grep(config) {
        eprintln!("Something went wrong reading the file: {:?}", err);
        process::exit(2);
    };
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        let expected = SearchResult {
            line_no: 2,
            line: "safe, fast, productive.".to_string(),
        };
        let result = search(
            query.to_string(), contents.to_string(), true);
        assert_eq!(1, result.len());
        assert_eq!(expected, result[0]);
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        let expected = vec![
            SearchResult {
                line_no: 1,
                line: "Rust:".to_string(),
            },
            SearchResult {
                line_no: 4,
                line: "Trust me.".to_string(),
            },
        ];
        let result = search(
            query.to_string(), contents.to_string(), false);
        assert_eq!(2, result.len());
        assert_eq!(expected[0], result[0]);
        assert_eq!(expected[1], result[1]);
    }
}