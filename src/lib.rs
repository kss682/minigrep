use std::fs;
use std::error::Error;

pub struct Config{
    pub query: String,
    pub file_path: String
}

impl Config {
    pub fn new(args: Vec<String>) -> Result<Config, &'static str>{
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        else if args.len() > 3 {
            return Err("Too many arguments");
        }

        let query = args.get(1).expect("Invalid query");
        let file_path = args.get(2).expect("Invalid file path");

        Ok(Config{
            query: query.to_string(),
            file_path: file_path.to_string()
        })
    }
}


pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    // Read from the file
    let file_contents = fs::read_to_string(config.file_path)?;
    for line in search(config.query, file_contents){
        println!("{line}");
    }
    Ok(())
}


pub fn search(query: String, file_contents: String)-> Vec<String> {
    let mut results: Vec<String> = Vec::new();
    let file_contents_list: Vec<&str> = file_contents.split("\n").collect();

    for content in file_contents_list.iter(){
        if content.contains(&query){
            results.push(content.to_string());
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result(){
        let query = "duct".to_string();
        let contents = "\
Rust:
safe, fast, productive.
Pick three.".to_string();
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn no_result(){
        let query = "check".to_string();
        let contents = "\
Rust:
safe, fast, productive.
Pick three.".to_string();
        assert_eq!(0, search(query, contents).len());
    }
}

