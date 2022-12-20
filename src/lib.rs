use std::fs;
use std::collections::HashMap;
use std::error::Error;

pub struct Config{
    pub query: String,
    pub file_path: String,
    pub case_insensitive: bool,
    pub line_number: bool
}

impl Config {
    pub fn new(args: Vec<String>) -> Result<Config, &'static str>{
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let query = args.get(1).expect("Invalid query");
        let file_path = args.get(2).expect("Invalid file path");
        let mut case_insensitive = false;
        let mut line_number = false;

        if args.len() > 3 {
            let options = args.get(3).expect("Invalid options");
            if options.contains("i"){
                case_insensitive = true;
            }
            if options.contains("n"){
                line_number = true;
            }
        }
        Ok(Config{
            query: query.to_string(),
            file_path: file_path.to_string(),
            case_insensitive: case_insensitive,
            line_number: line_number
        })
    }
}


pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    // Read from the file
    let file_contents = fs::read_to_string(&config.file_path)?;
    for (no, line) in &search(config.query, file_contents, config.case_insensitive){
        if config.line_number == true {
            println!("{}::{}::{}", config.file_path, no, line);
        }
        else{
            println!("{line}");
        }
    }
    Ok(())
}


pub fn search(query: String, file_contents: String, case_insensitive: bool)-> HashMap<i32, String> {
    let mut results = HashMap::new();
    let file_contents_list: Vec<&str> = file_contents.split("\n").collect();

    let mut counter: i32 = 1;
    for content in file_contents_list.iter(){
        if case_insensitive == true {
            if content.to_lowercase().contains(&query.to_lowercase()){
                results.insert(counter, content.to_string());
            }
        }
        else if content.contains(&query){
            results.insert(counter, content.to_string());
        }
        counter += 1;
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result(){
        let case_insensitive = true;
        let query = "duct".to_string();
        let contents = "Rust:
        safe, fast, productive.
        Pick three.".to_string();
        assert_eq!("safe, fast, productive.", search(query, contents, case_insensitive).get(&2).unwrap().trim());
    }

    #[test]
    fn no_result(){
        let case_insensitive = true;
        let query = "check".to_string();
        let contents = "Rust:
        safe, fast, productive.
        Pick three.".to_string();
        assert_eq!(0, search(query, contents, case_insensitive).len());
    }

    #[test]
    fn case_insensitive_query(){
        let case_insensitive = true;
        let query = "Duct".to_string();
        let contents = "Rust:
        safe, fast, productive.
        Pick three.".to_string();
    
        assert_eq!("safe, fast, productive.", search(query, contents, case_insensitive).get(&2).unwrap().trim());
    }

    #[test]
    fn case_sensitive_query(){
        let case_insensitive = false;
        let query = "Duct".to_string();
        let contents = "\
        Rust:
        safe, fast, productive.
        Pick three.".to_string();
    
        assert_eq!(0, search(query, contents, case_insensitive).len());
    }

    #[test]
    fn case_insensitive_contents(){
        let case_insensitive = true;
        let query = "duct".to_string();
        let contents = "Rust:
        safe, fast, proDuctive.
        Pick three.".to_string();
     
        assert_eq!("safe, fast, proDuctive.", search(query, contents, case_insensitive).get(&2).unwrap().trim());
    }
}

