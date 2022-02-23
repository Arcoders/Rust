use std::fs;
use std::error::Error;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content: String = fs::read_to_string(config.filename)?;
    println!("{}", content);
    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query: String = args[1].clone();
        let filename: String = args[2].clone();
        Ok(Config { query, filename })
    }
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let query: String = query.to_lowercase();
    let mut results = Vec::new();
    for line in content.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn one_result() {
        let query: &str = "of the printing";
        let content: &str = "
Lorem Ipsum is simply dummy text
of the printing
and typesetting industry.";
        assert_eq!(vec!["of the printing"], search(query, content));
    }

    #[test]
    fn case_sensitive() {
        let query: &str = "ipsum";
        let content: &str = "
Lorem Ipsum is simply dummy text
of the printing
and typesetting industry.";
        assert_eq!(vec!["Lorem Ipsum is simply dummy text"], search(query, content));
    }
}