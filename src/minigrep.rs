use std::env;
use std::error::Error;
use std::fs;
use std::process;

struct Config {
    query: String,
    filename: String,
    case_sensitive: bool,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config { query, filename, case_sensitive })
    }
}


fn grep(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    //println!("With text:\n{}", contents);
    for result in search(&config.query, &contents, config.case_sensitive) {
        println!("{}: {}", result.line_no, result.line);
    }
    Ok(())
}


#[derive(Debug, PartialEq)]
struct SearchResult {
    line_no: i32,
    line: String,
}

fn search(query: &str, contents: &str, case_sensitive: bool) -> Vec<SearchResult> {
    let mut results = Vec::new();
    let mut lineno = 1;
    let query = if case_sensitive { query.to_string() } else { query.to_lowercase() };
    for line in contents.lines() {
        let mut push = false;
        if case_sensitive && line.contains(&query) {
           push = true;
        } else if !case_sensitive && line.to_lowercase().contains(&query) {
           push = true;
        }
        if push {
            results.push(SearchResult { line_no: lineno, line: line.to_string() });
        }
        lineno += 1;
    }
    results
}

pub fn run() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
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
        let result = search(query, contents, true);
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
        let result = search(query, contents, false);
        assert_eq!(2, result.len());
        assert_eq!(expected[0], result[0]);
        assert_eq!(expected[1], result[1]);
    }
}