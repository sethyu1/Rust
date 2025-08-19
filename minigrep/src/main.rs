use std::process;
use std::env;
use std::fs;
use std::error::Error;
use minigrep::search;
use minigrep::search_case_insensitive;
use minigrep::Config;


fn main() {
    
    // The env::args function returns an iterator! 
    let config = Config::build(env::args()).unwrap_or_else(|err|{
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // println!("Searching for {}", config.query);
    // println!("In file {}", config.file_path);

    if let Err(e) = run(config) {
        eprintln!("Apoplication error: {}", e);
        process::exit(1);
        }
    }

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    
    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}'");
    }
    Ok(())
}
