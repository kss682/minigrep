/*
    A programme to search for string in file.
    Options:
        -i: case insensitive search
        -n: line number and file name 
        -r: recursive search on subfolders
            Using -r includes -n options
        -h: Help ?
*/
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = minigrep::Config::new(args).unwrap_or_else(|err| {
        println!("Problem parsing arguements {err}");
        process::exit(1);
    });

    minigrep::run(config).unwrap_or_else(
        |err| {
            println!("Problem reading file {err}");
            process::exit(1);
        }
    );


}
