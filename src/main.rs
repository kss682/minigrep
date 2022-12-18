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
